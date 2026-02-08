@echo off
echo =======================================
echo Rust Environment Check
echo =======================================
echo.

echo [Step 1/5] Checking Rust...
echo.
where rustc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [FAIL] Rust is NOT installed
    echo Install from: https://rustup.rs/
    echo.
) else (
    echo [OK] Rust is installed
    rustc --version
    cargo --version
    echo.
)

echo [Step 2/5] Checking Rust toolchain...
echo.
rustup show
echo.

echo [Step 3/5] Checking Visual Studio Build Tools...
echo.
where link.exe >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [FAIL] Visual Studio Build Tools NOT found
    echo.
    echo *** THIS IS YOUR PROBLEM! ***
    echo.
    echo SOLUTION:
    echo 1. Download: https://aka.ms/vs/17/release/vs_buildtools.exe
    echo 2. Install "Desktop development with C++"
    echo 3. Reboot computer
    echo.
) else (
    echo [OK] Build Tools found at:
    where link.exe
    echo.
)

echo [Step 4/5] Checking Windows SDK...
echo.
if exist "C:\Program Files (x86)\Windows Kits\10\" (
    echo [OK] Windows 10 SDK found
) else (
    echo [FAIL] Windows SDK not found
)
echo.

echo [Step 5/5] Environment Variables...
echo.
if defined LIB (
    echo [OK] LIB variable is set
) else (
    echo [FAIL] LIB is NOT set
    echo You may need to run from Developer Command Prompt
)
echo.

echo =======================================
echo SUMMARY
echo =======================================
echo.

where rustc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Status: FAILED - Rust not installed
    echo Action: Install Rust from https://rustup.rs/
    goto end
)

where link.exe >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Status: FAILED - Build Tools missing
    echo.
    echo *** ACTION REQUIRED ***
    echo.
    echo Step 1: Download Build Tools
    echo   https://aka.ms/vs/17/release/vs_buildtools.exe
    echo.
    echo Step 2: Run the installer
    echo.
    echo Step 3: Select "Desktop development with C++"
    echo.
    echo Step 4: Click Install and wait (20-40 minutes)
    echo.
    echo Step 5: Reboot your computer
    echo.
    echo Step 6: Run this script again to verify
    echo.
    echo See FIX_LINKER_ERROR.md for detailed instructions
    goto end
)

echo Status: READY - All tools installed!
echo.
echo You can now build the project with:
echo   cargo build --release
echo.
echo Or simply run:
echo   build.bat
echo.

:end
echo.
pause
