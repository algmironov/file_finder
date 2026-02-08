@echo off
chcp 65001 > nul
echo =======================================
echo FileFinder Build Script
echo =======================================
echo.

REM Check Rust installation
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Rust is not installed!
    echo.
    echo Install Rust from https://rustup.rs/
    echo.
    pause
    exit /b 1
)

echo [1/3] Checking dependencies...
cargo --version
rustc --version
echo.

echo [2/3] Building project in release mode...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Build failed!
    echo.
    echo Try updating Rust: rustup update
    pause
    exit /b 1
)

echo.
echo [3/3] Build successful!
echo.
echo Executable: target\release\file_finder.exe
echo Size: 
for %%A in (target\release\file_finder.exe) do echo %%~zA bytes

echo.
echo =======================================
echo Build completed!
echo =======================================
echo.
echo Run the program?
choice /C YN /M "Choose Y (yes) or N (no)"

if errorlevel 2 goto end
if errorlevel 1 goto run

:run
echo.
echo Running file_finder.exe --help
echo.
target\release\file_finder.exe --help

:end
pause
