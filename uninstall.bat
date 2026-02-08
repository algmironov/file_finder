@echo off
echo =======================================
echo FileFinder Uninstallation
echo =======================================
echo.

echo Searching for installed versions...
echo.

set FOUND=0

REM Check cargo install location
if exist "%USERPROFILE%\.cargo\bin\file_finder.exe" (
    echo [FOUND] Cargo installation: %USERPROFILE%\.cargo\bin\file_finder.exe
    set FOUND=1
)

REM Check System32
if exist "C:\Windows\System32\file_finder.exe" (
    echo [FOUND] System32: C:\Windows\System32\file_finder.exe
    set FOUND=1
)

REM Check user bin
if exist "%USERPROFILE%\bin\file_finder.exe" (
    echo [FOUND] User bin: %USERPROFILE%\bin\file_finder.exe
    set FOUND=1
)

if %FOUND%==0 (
    echo [INFO] No installations found.
    echo.
    pause
    exit /b 0
)

echo.
echo =======================================
echo Uninstall Options
echo =======================================
echo.
echo 1. Uninstall ALL found versions
echo 2. Uninstall Cargo version only
echo 3. Uninstall System32 version only
echo 4. Uninstall User bin version only
echo 5. Cancel
echo.

choice /C 12345 /M "Select option"

if errorlevel 5 goto cancel
if errorlevel 4 goto uninstall_user_bin
if errorlevel 3 goto uninstall_system32
if errorlevel 2 goto uninstall_cargo
if errorlevel 1 goto uninstall_all

:uninstall_all
echo.
echo Uninstalling all versions...
echo.

if exist "%USERPROFILE%\.cargo\bin\file_finder.exe" (
    echo Removing Cargo version...
    cargo uninstall file_finder
)

if exist "C:\Windows\System32\file_finder.exe" (
    echo Removing System32 version...
    del "C:\Windows\System32\file_finder.exe"
)

if exist "%USERPROFILE%\bin\file_finder.exe" (
    echo Removing User bin version...
    del "%USERPROFILE%\bin\file_finder.exe"
)

goto done

:uninstall_cargo
echo.
echo Uninstalling Cargo version...
cargo uninstall file_finder
goto done

:uninstall_system32
echo.
echo Uninstalling System32 version...
echo This may require administrator rights.
del "C:\Windows\System32\file_finder.exe"
goto done

:uninstall_user_bin
echo.
echo Uninstalling User bin version...
del "%USERPROFILE%\bin\file_finder.exe"
goto done

:done
echo.
echo =======================================
echo Uninstallation Complete!
echo =======================================
echo.

where file_finder >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [WARNING] 'file_finder' still found in PATH
    where file_finder
    echo.
    echo You may have multiple installations or need to restart terminal.
) else (
    echo [OK] 'file_finder' removed from system
)

echo.
pause
exit /b 0

:cancel
echo.
echo Uninstallation cancelled.
echo.
pause
exit /b 0
