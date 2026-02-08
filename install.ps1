# FileFinder Installation Script (PowerShell)

Write-Host "=======================================" -ForegroundColor Cyan
Write-Host "FileFinder Installation" -ForegroundColor Cyan
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host ""

# Check if built
if (-not (Test-Path "target\release\file_finder.exe")) {
    Write-Host "[1/3] Building release version..." -ForegroundColor Yellow
    Write-Host "This may take a few minutes..." -ForegroundColor Gray
    Write-Host ""
    
    cargo build --release
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host ""
        Write-Host "[ERROR] Build failed!" -ForegroundColor Red
        Write-Host "Run build_auto.bat first to build the project." -ForegroundColor Red
        Read-Host "Press Enter to exit"
        exit 1
    }
} else {
    Write-Host "[1/3] Using existing build..." -ForegroundColor Green
}

Write-Host ""
Write-Host "[2/3] Choose installation method:" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. Install via Cargo (recommended)" -ForegroundColor White
Write-Host "   - Installs to: $env:USERPROFILE\.cargo\bin\" -ForegroundColor Gray
Write-Host "   - No admin rights needed" -ForegroundColor Gray
Write-Host "   - Easy to update/uninstall" -ForegroundColor Gray
Write-Host ""
Write-Host "2. Copy to Windows\System32" -ForegroundColor White
Write-Host "   - Requires admin rights" -ForegroundColor Gray
Write-Host "   - Simple and fast" -ForegroundColor Gray
Write-Host ""
Write-Host "3. Copy to user bin folder" -ForegroundColor White
Write-Host "   - Installs to: $env:USERPROFILE\bin\" -ForegroundColor Gray
Write-Host "   - No admin rights needed" -ForegroundColor Gray
Write-Host "   - Will add to PATH automatically" -ForegroundColor Gray
Write-Host ""

$choice = Read-Host "Select option (1, 2, or 3)"

switch ($choice) {
    "1" {
        Write-Host ""
        Write-Host "Installing via Cargo..." -ForegroundColor Yellow
        
        cargo install --path . --force
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "[ERROR] Installation failed!" -ForegroundColor Red
            Read-Host "Press Enter to exit"
            exit 1
        }
        
        Write-Host ""
        Write-Host "[OK] Installed to: $env:USERPROFILE\.cargo\bin\file_finder.exe" -ForegroundColor Green
    }
    
    "2" {
        Write-Host ""
        Write-Host "Installing to Windows\System32..." -ForegroundColor Yellow
        Write-Host "This requires administrator rights." -ForegroundColor Gray
        Write-Host ""
        
        # Check if running as admin
        $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
        
        if (-not $isAdmin) {
            Write-Host "[WARNING] Not running as administrator!" -ForegroundColor Yellow
            Write-Host "Attempting to elevate..." -ForegroundColor Gray
            
            Start-Process powershell -Verb RunAs -ArgumentList "-NoExit", "-Command", "Copy-Item 'target\release\file_finder.exe' 'C:\Windows\System32\' -Force"
            
            Write-Host "Please complete the installation in the elevated window." -ForegroundColor Yellow
            Read-Host "Press Enter to continue"
            exit 0
        }
        
        Copy-Item "target\release\file_finder.exe" "C:\Windows\System32\" -Force
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "[ERROR] Copy failed!" -ForegroundColor Red
            Read-Host "Press Enter to exit"
            exit 1
        }
        
        Write-Host "[OK] Installed to: C:\Windows\System32\file_finder.exe" -ForegroundColor Green
    }
    
    "3" {
        Write-Host ""
        Write-Host "Installing to user bin folder..." -ForegroundColor Yellow
        
        $binPath = "$env:USERPROFILE\bin"
        
        if (-not (Test-Path $binPath)) {
            New-Item -ItemType Directory -Path $binPath | Out-Null
            Write-Host "Created folder: $binPath" -ForegroundColor Gray
        }
        
        Copy-Item "target\release\file_finder.exe" "$binPath\" -Force
        
        Write-Host "[OK] Installed to: $binPath\file_finder.exe" -ForegroundColor Green
        Write-Host ""
        Write-Host "Adding to PATH..." -ForegroundColor Yellow
        
        # Get current user PATH
        $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
        
        # Check if already in PATH
        if ($currentPath -notlike "*$binPath*") {
            $newPath = "$currentPath;$binPath"
            [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
            
            Write-Host "[OK] Added to PATH" -ForegroundColor Green
            Write-Host "You need to restart PowerShell for changes to take effect" -ForegroundColor Yellow
        } else {
            Write-Host "[OK] Already in PATH" -ForegroundColor Green
        }
    }
    
    default {
        Write-Host "[ERROR] Invalid choice!" -ForegroundColor Red
        Read-Host "Press Enter to exit"
        exit 1
    }
}

Write-Host ""
Write-Host "[3/3] Verifying installation..." -ForegroundColor Yellow
Write-Host ""

# Refresh PATH for current session
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")

$findExe = Get-Command file_finder -ErrorAction SilentlyContinue

if ($findExe) {
    Write-Host "[SUCCESS] Installation verified!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Installed at: $($findExe.Source)" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Testing..." -ForegroundColor Gray
    & file_finder --version
} else {
    Write-Host "[WARNING] 'file_finder' not found in current session" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "This is normal. Please:" -ForegroundColor Gray
    Write-Host "1. Close and reopen PowerShell" -ForegroundColor Gray
    Write-Host "2. Then run: file_finder --help" -ForegroundColor Gray
}

Write-Host ""
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host "Installation Complete!" -ForegroundColor Green
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Usage examples:" -ForegroundColor White
Write-Host "  file_finder --help" -ForegroundColor Gray
Write-Host "  file_finder --min-size 500MB" -ForegroundColor Gray
Write-Host "  file_finder -e mp4,mkv --duplicates" -ForegroundColor Gray
Write-Host ""

Read-Host "Press Enter to exit"
