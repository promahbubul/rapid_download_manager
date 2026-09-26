# PowerShell MSIX Build Script for Rapid Download Manager
$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $PSScriptRoot
$DistApp = Join-Path $ProjectRoot "dist\RapidDownloadManager"
$MsixSrc = Join-Path $ProjectRoot "msix"
$StagingDir = Join-Path $ProjectRoot "dist\msix_staging"
$InstallerDir = Join-Path $ProjectRoot "dist\installer"
$OutputMsix = Join-Path $InstallerDir "RapidDownloadManager_v1.0.0.msix"

Write-Host "==> Staging MSIX Package directory..." -ForegroundColor Cyan
if (Test-Path $StagingDir) { Remove-Item -Recurse -Force $StagingDir }
New-Item -ItemType Directory -Force -Path $StagingDir | Out-Null

Copy-Item -Recurse -Force "$DistApp\*" $StagingDir
Copy-Item -Force (Join-Path $MsixSrc "AppxManifest.xml") $StagingDir
Copy-Item -Recurse -Force (Join-Path $MsixSrc "Assets") $StagingDir

# Search for MakeAppx
$MakeAppx = (Get-Command makeappx.exe -ErrorAction SilentlyContinue).Source
if (-not $MakeAppx) {
    $KitCandidate = Get-ChildItem "C:\Program Files (x86)\Windows Kits\10\bin\*\x64\makeappx.exe" -ErrorAction SilentlyContinue | Select-Object -Last 1
    if ($KitCandidate) { $MakeAppx = $KitCandidate.FullName }
}

if ($MakeAppx) {
    Write-Host "==> Packaging with MakeAppx ($MakeAppx)..." -ForegroundColor Green
    & $MakeAppx pack /d $StagingDir /p $OutputMsix /o /nv
    Write-Host "==> SUCCESS: $OutputMsix generated." -ForegroundColor Green
} else {
    Write-Host "==> MakeAppx not found. Run 'python msix\build_msix.py' or install Windows SDK." -ForegroundColor Yellow
}
