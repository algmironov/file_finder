@echo off
echo =======================================
echo FileFinder Installation Script
echo =======================================
echo.

REM Check if already built
if not exist "target\release\file_finder.exe" (
    echo [1/3] Building release version...
    echo This may take a few minutes...
    echo.
    cargo build --release
    
    if %ERRORLEVEL% NEQ 0 (
        echo.
        echo [ERROR] Build failed!
        echo Run build_auto.bat first to build the project.
        pause
        exit /b 1
    )
) else (
    echo [1/3] Using existing build...
)

echo.
echo [2/3] Choose installation method:
echo.
echo 1. Install via Cargo (recommended)
echo    - Installs to: %USERPROFILE%\.cargo\bin\
echo    - No admin rights needed
echo    - Easy to update/uninstall
echo.
echo 2. Copy to Windows\System32
echo    - Requires admin rights
echo    - Simple and fast
echo.
echo 3. Copy to user bin folder
echo    - Installs to: %USERPROFILE%\bin\
echo    - No admin rights needed
echo    - Need to add to PATH manually
echo.

choice /C 123 /M "Select option (1, 2, or 3)"

if errorlevel 3 goto option3
if errorlevel 2 goto option2
if errorlevel 1 goto option1

:option1
echo.
echo Installing via Cargo...
cargo install --path . --force

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Installation failed!
    pause
    exit /b 1
)

echo.
echo [OK] Installed to: %USERPROFILE%\.cargo\bin\file_finder.exe
goto verify

:option2
echo.
echo Installing to Windows\System32...
echo This requires administrator rights.
echo.

copy /Y target\release\file_finder.exe C:\Windows\System32\

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Copy failed! Run as Administrator.
    pause
    exit /b 1
)

echo [OK] Installed to: C:\Windows\System32\file_finder.exe
goto verify

:option3
echo.
echo Installing to user bin folder...

if not exist "%USERPROFILE%\bin\" (
    mkdir "%USERPROFILE%\bin"
    echo Created folder: %USERPROFILE%\bin
)

copy /Y target\release\file_finder.exe "%USERPROFILE%\bin\"

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Copy failed!
    pause
    exit /b 1
)

echo [OK] Installed to: %USERPROFILE%\bin\file_finder.exe
echo.
echo IMPORTANT: Add to PATH manually:
echo 1. Win + X -^> System
echo 2. Advanced system settings
echo 3. Environment Variables
echo 4. Edit "Path" for your user
echo 5. Add: %USERPROFILE%\bin
echo 6. Restart PowerShell/CMD
echo.
pause

goto verify

:verify
echo.
echo [3/3] Verifying installation...
echo.

where file_finder >nul 2>nul

if %ERRORLEVEL% EQU 0 (
    echo [SUCCESS] Installation verified!
    echo.
    where file_finder
    echo.
    echo Testing...
    file_finder --version
) else (
    echo [WARNING] 'file_finder' not found in PATH
    echo.
    echo You may need to:
    echo 1. Restart your terminal (PowerShell/CMD)
    echo 2. Or add the install location to PATH
)

echo.
echo =======================================
echo Installation Complete!
echo =======================================
echo.
echo You can now run 'file_finder' from anywhere:
echo   file_finder --help
echo   file_finder --min-size 500MB
echo   file_finder -e mp4,mkv --duplicates
echo.

pause
