@echo off
chcp 65001 > nul
echo =======================================
echo Rust Update Script
echo =======================================
echo.

echo Current version:
rustc --version
echo.

echo Updating Rust to the latest version...
echo This may take a few minutes...
echo.

rustup update stable

echo.
echo =======================================
echo Update completed!
echo =======================================
echo.

echo New version:
rustc --version
echo.

echo You can now run build.bat to compile the project.
echo.

pause
