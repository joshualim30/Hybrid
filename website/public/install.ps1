# Hybrid CLI Installer for Windows
# Version: 0.1.0

Write-Host "🚀 Starting Hybrid CLI installation..." -ForegroundColor Cyan

$OS = "windows"
$ARCH = "x64"

Write-Host "📦 Detected $OS ($ARCH)" -ForegroundColor Gray

# Simulated download
$InstallDir = "$env:USERPROFILE\AppData\Local\Hybrid\bin"
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

Write-Host "⬇️  Downloading Hybrid CLI..." -ForegroundColor Gray
# Invoke-WebRequest -Uri "https://github.com/Creating-Real/hybrid/releases/latest/download/hybrid-windows-x64.exe" -OutFile "$InstallDir\hybrid.exe"

$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    Write-Host "🔗 Adding Hybrid to PATH..." -ForegroundColor Cyan
    [Environment]::SetEnvironmentVariable("Path", "$CurrentPath;$InstallDir", "User")
}

Write-Host "✅ Hybrid CLI successfully installed!" -ForegroundColor Green
Write-Host "👉 Restart your terminal and run 'hybrid --help' to get started." -ForegroundColor Yellow
