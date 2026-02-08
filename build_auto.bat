@echo off
echo =======================================
echo FileFinder Auto Build
echo =======================================
echo.

echo [1/4] Searching for Visual Studio...
echo.

REM Try to find and setup VS environment
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"

if exist "%VSWHERE%" (
    echo Using vswhere to locate Visual Studio...
    for /f "usebackq tokens=*" %%i in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do (
        set "VSINSTALLDIR=%%i"
    )
    
    if defined VSINSTALLDIR (
        if exist "%VSINSTALLDIR%\VC\Auxiliary\Build\vcvars64.bat" (
            echo [OK] Found Visual Studio at: %VSINSTALLDIR%
            call "%VSINSTALLDIR%\VC\Auxiliary\Build\vcvars64.bat" >nul 2>nul
            goto :build
        )
    )
)

REM Manual search for common locations
set "VCVARSALL="

if exist "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    set "VCVARSALL=C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
    goto :setup
)

if exist "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" (
    set "VCVARSALL=C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
    goto :setup
)

if exist "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat" (
    set "VCVARSALL=C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat"
    goto :setup
)

if exist "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    set "VCVARSALL=C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
    goto :setup
)

if exist "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat" (
    set "VCVARSALL=C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
    goto :setup
)

echo [ERROR] Could not find Visual Studio Build Tools!
echo.
echo Possible reasons:
echo 1. Build Tools not installed
echo 2. "Desktop development with C++" component missing
echo.
echo Solutions:
echo 1. Open Visual Studio Installer
echo 2. Click "Modify"
echo 3. Ensure "Desktop development with C++" is checked
echo 4. Click "Modify" to install
echo.
echo Or download from: https://aka.ms/vs/17/release/vs_buildtools.exe
echo.
pause
exit /b 1

:setup
echo [OK] Found Visual Studio
call "%VCVARSALL%" >nul 2>nul

:build
echo.
echo [2/4] Checking Rust...
cargo --version >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Rust not found!
    echo Install from: https://rustup.rs/
    pause
    exit /b 1
)
echo [OK] Rust found
rustc --version
cargo --version
echo.

echo [3/4] Building project (release mode)...
echo This may take a few minutes on first build...
echo.

cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Build failed!
    echo.
    echo Check the error messages above.
    echo.
    pause
    exit /b 1
)

echo.
echo [4/4] Build successful!
echo.
echo Executable: target\release\file_finder.exe

if exist "target\release\file_finder.exe" (
    for %%A in (target\release\file_finder.exe) do echo Size: %%~zA bytes
)

echo.
echo =======================================
echo Build completed successfully!
echo =======================================
echo.
echo Run the program?
choice /C YN /M "Y = Yes, N = No" /T 10 /D N

if errorlevel 2 goto end
if errorlevel 1 goto run

:run
echo.
target\release\file_finder.exe --help

:end
echo.
pause
