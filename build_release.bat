@echo off
setlocal enabledelayedexpansion

echo =======================================
echo FileFinder Release Builder
echo =======================================
echo.

REM Get version from user
set /p VERSION="Enter version (e.g., 1.0.0): "

if "%VERSION%"=="" (
    echo Error: Version cannot be empty
    pause
    exit /b 1
)

set NAME=file_finder-v%VERSION%-windows-x64

echo Building FileFinder v%VERSION%...
echo.

REM Build release binary
echo [1/5] Building release binary...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Build failed!
    pause
    exit /b 1
)

echo [OK] Build complete
echo.

REM Create release directory
echo [2/5] Creating release directory...
if exist "release\%NAME%" rmdir /s /q "release\%NAME%"
mkdir "release\%NAME%"

echo [OK] Directory created
echo.

REM Copy files
echo [3/5] Copying files...

copy "target\release\file_finder.exe" "release\%NAME%\" >nul
copy "README.md" "release\%NAME%\README.txt" >nul
copy "EXAMPLES.md" "release\%NAME%\EXAMPLES.txt" >nul
copy "LICENSE" "release\%NAME%\LICENSE.txt" >nul
copy "CHANGELOG.md" "release\%NAME%\CHANGELOG.txt" >nul

echo [OK] Files copied
echo.

REM Create archive
echo [4/5] Creating archive...

cd release
if exist "%NAME%.zip" del "%NAME%.zip"

powershell -Command "Compress-Archive -Path '%NAME%' -DestinationPath '%NAME%.zip' -Force"

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Archive creation failed!
    cd ..
    pause
    exit /b 1
)

cd ..

echo [OK] Archive created
echo.

REM Calculate checksum
echo [5/5] Calculating checksum...

powershell -Command "$hash = (Get-FileHash -Path 'release\%NAME%.zip' -Algorithm SHA256).Hash; $hash | Out-File -FilePath 'release\%NAME%.zip.sha256' -NoNewline"

echo [OK] Checksum calculated
echo.

REM Display results
echo =======================================
echo Release build complete!
echo =======================================
echo.
echo Files created:
echo   release\%NAME%.zip
echo   release\%NAME%.zip.sha256
echo.

for %%A in (release\%NAME%.zip) do (
    echo Size: %%~zA bytes
)

echo.
echo Checksum:
type release\%NAME%.zip.sha256
echo.
echo.

REM Show next steps
echo =======================================
echo Next Steps:
echo =======================================
echo.
echo 1. Test the release:
echo    - Extract release\%NAME%.zip
echo    - Run file_finder.exe
echo    - Verify it works correctly
echo.
echo 2. Create Git tag:
echo    git tag -a v%VERSION% -m "Release version %VERSION%"
echo    git push origin v%VERSION%
echo.
echo 3. Create GitHub Release:
echo    - Go to https://github.com/yourusername/file_finder/releases
echo    - Click "Create a new release"
echo    - Select tag v%VERSION%
echo    - Upload release\%NAME%.zip
echo    - Upload release\%NAME%.zip.sha256
echo    - Add release notes from CHANGELOG.md
echo.

pause
