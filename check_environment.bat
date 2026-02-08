@echo off
echo =======================================
echo Rust Environment Check
echo =======================================
echo.

echo [1/5] Checking Rust installation...
echo.
where rustc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [✗] Rust is NOT installed
    echo     Install from: https://rustup.rs/
    echo.
) else (
    echo [✓] Rust is installed
    rustc --version
    cargo --version
    echo.
)

echo [2/5] Checking Rust toolchain...
echo.
rustup show
echo.

echo [3/5] Checking Visual Studio Build Tools...
echo.
where link.exe >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [✗] Visual Studio Build Tools NOT found
    echo     This is the PROBLEM!
    echo.
    echo     SOLUTION:
    echo     1. Download: https://aka.ms/vs/17/release/vs_buildtools.exe
    echo     2. Install "Desktop development with C++"
    echo     3. Reboot computer
    echo.
) else (
    echo [✓] Build Tools found:
    where link.exe
    where cl.exe 2>nul
    echo.
)

echo [4/5] Checking Windows SDK...
echo.
if exist "C:\Program Files (x86)\Windows Kits\10\" (
    echo [✓] Windows 10 SDK found
    dir "C:\Program Files (x86)\Windows Kits\10\Lib\" | findstr /C:"10.0."
) else (
    echo [✗] Windows SDK not found
)
echo.

echo [5/5] Environment Variables...
echo.
if defined LIB (
    echo [✓] LIB is set
    echo LIB=%LIB%
) else (
    echo [✗] LIB is NOT set
    echo     You may need to run from Developer Command Prompt
)
echo.

echo =======================================
echo Summary
echo =======================================
echo.

where rustc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Status: [✗] FAILED - Rust not installed
    echo Action: Install Rust from https://rustup.rs/
    goto end
)

where link.exe >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Status: [✗] FAILED - Visual Studio Build Tools missing
    echo.
    echo ACTION REQUIRED:
    echo 1. Download Build Tools: https://aka.ms/vs/17/release/vs_buildtools.exe
    echo 2. Run installer
    echo 3. Select "Desktop development with C++"
    echo 4. Click Install
    echo 5. Reboot computer
    echo 6. Run this script again
    echo.
    echo See FIX_LINKER_ERROR.md for detailed instructions
    goto end
)

echo Status: [✓] READY - All tools are installed!
echo.
echo You can now build the project:
echo     cargo build --release
echo.

:end
echo.
pause
