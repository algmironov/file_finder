@echo off
echo =======================================
echo Visual Studio Environment Setup
echo =======================================
echo.

echo Searching for Visual Studio installations...
echo.

REM Check VS 2022
if exist "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    echo [FOUND] Visual Studio 2022 Build Tools
    call "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
    goto :found
)

if exist "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat" (
    echo [FOUND] Visual Studio 2022 Community
    call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"
    goto :found
)

if exist "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat" (
    echo [FOUND] Visual Studio 2022 Professional
    call "C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat"
    goto :found
)

REM Check VS 2019
if exist "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    echo [FOUND] Visual Studio 2019 Build Tools
    call "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
    goto :found
)

if exist "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat" (
    echo [FOUND] Visual Studio 2019 Community
    call "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
    goto :found
)

REM Check VS 2017
if exist "C:\Program Files (x86)\Microsoft Visual Studio\2017\BuildTools\VC\Auxiliary\Build\vcvars64.bat" (
    echo [FOUND] Visual Studio 2017 Build Tools
    call "C:\Program Files (x86)\Microsoft Visual Studio\2017\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
    goto :found
)

echo [NOT FOUND] Could not locate Visual Studio installation
echo.
echo Please check if "Desktop development with C++" is installed:
echo 1. Open Visual Studio Installer
echo 2. Click "Modify" on your installation
echo 3. Ensure "Desktop development with C++" is checked
echo 4. Click "Modify" to install if missing
echo.
pause
exit /b 1

:found
echo.
echo =======================================
echo Environment configured!
echo =======================================
echo.

echo Verifying tools...
where link.exe
where cl.exe
echo.

echo You can now build the project:
echo   cargo build --release
echo.

REM Keep the window open with environment set
cmd /k
