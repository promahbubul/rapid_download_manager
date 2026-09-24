# Rapid Download Manager - Automated Browser Integration Script
param(
    [switch]$LaunchBrowsers = $false,
    [switch]$Silent = $false
)

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
$ExtensionDir = Join-Path $ScriptDir "extension"

if (-not (Test-Path $ExtensionDir)) {
    $ExtensionDir = Join-Path (Split-Path -Parent $ScriptDir) "extension"
}

if (-not $Silent) {
    Write-Host "==========================================================" -ForegroundColor Cyan
    Write-Host "  ⚡ Rapid Download Manager - Auto Browser Integration" -ForegroundColor White
    Write-Host "==========================================================" -ForegroundColor Cyan
    Write-Host "Extension directory: $ExtensionDir" -ForegroundColor Gray
    Write-Host ""
}

$results = [System.Collections.Generic.List[PSCustomObject]]::new()

function Register-BrowserExt {
    param(
        [string]$BrowserName,
        [string]$RegKeyPath,
        [string]$ValueName,
        [string]$ValueData,
        [string]$BrowserExe,
        [string]$ExtUrl
    )

    $isInstalled = $false
    if ($BrowserExe -and (Test-Path $BrowserExe)) {
        $isInstalled = $true
    }

    try {
        if (-not (Test-Path $RegKeyPath)) {
            New-Item -Path $RegKeyPath -Force | Out-Null
        }
        Set-ItemProperty -Path $RegKeyPath -Name $ValueName -Value $ValueData -Force | Out-Null
        $status = "Integrated"
    } catch {
        $status = "Failed: $_"
    }

    $results.Add([PSCustomObject]@{
        Browser   = $BrowserName
        Installed = $isInstalled
        Status    = $status
        Exe       = $BrowserExe
        ExtUrl    = $ExtUrl
    })
}

# 1. Google Chrome
$chromeExe = @(
    "$env:LOCALAPPDATA\Google\Chrome\Application\chrome.exe",
    "$env:ProgramFiles\Google\Chrome\Application\chrome.exe",
    "${env:ProgramFiles(x86)}\Google\Chrome\Application\chrome.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1

Register-BrowserExt -BrowserName "Google Chrome" `
    -RegKeyPath "HKCU:\Software\Google\Chrome\Extensions\rapid_download_manager" `
    -ValueName "path" `
    -ValueData $ExtensionDir `
    -BrowserExe $chromeExe `
    -ExtUrl "chrome://extensions"

# 2. Microsoft Edge
$edgeExe = @(
    "$env:ProgramFiles\Microsoft\Edge\Application\msedge.exe",
    "${env:ProgramFiles(x86)}\Microsoft\Edge\Application\msedge.exe",
    "$env:LOCALAPPDATA\Microsoft\Edge\Application\msedge.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1

Register-BrowserExt -BrowserName "Microsoft Edge" `
    -RegKeyPath "HKCU:\Software\Microsoft\Edge\Extensions\rapid_download_manager" `
    -ValueName "path" `
    -ValueData $ExtensionDir `
    -BrowserExe $edgeExe `
    -ExtUrl "edge://extensions"

# 3. Brave Browser
$braveExe = @(
    "$env:ProgramFiles\BraveSoftware\Brave-Browser\Application\brave.exe",
    "${env:ProgramFiles(x86)}\BraveSoftware\Brave-Browser\Application\brave.exe",
    "$env:LOCALAPPDATA\BraveSoftware\Brave-Browser\Application\brave.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1

Register-BrowserExt -BrowserName "Brave Browser" `
    -RegKeyPath "HKCU:\Software\BraveSoftware\Brave-Browser\Extensions\rapid_download_manager" `
    -ValueName "path" `
    -ValueData $ExtensionDir `
    -BrowserExe $braveExe `
    -ExtUrl "brave://extensions"

# 4. Opera / Opera GX
$operaExe = @(
    "$env:LOCALAPPDATA\Programs\Opera\launcher.exe",
    "$env:LOCALAPPDATA\Programs\Opera GX\launcher.exe",
    "$env:ProgramFiles\Opera\launcher.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1

Register-BrowserExt -BrowserName "Opera / Opera GX" `
    -RegKeyPath "HKCU:\Software\Opera Software\Extensions\rapid_download_manager" `
    -ValueName "path" `
    -ValueData $ExtensionDir `
    -BrowserExe $operaExe `
    -ExtUrl "opera://extensions"

# 5. Mozilla Firefox
$firefoxExe = @(
    "$env:ProgramFiles\Mozilla Firefox\firefox.exe",
    "${env:ProgramFiles(x86)}\Mozilla Firefox\firefox.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1

Register-BrowserExt -BrowserName "Mozilla Firefox" `
    -RegKeyPath "HKCU:\Software\Mozilla\Firefox\Extensions" `
    -ValueName "rapid-downloader@promahbubul.com" `
    -ValueData $ExtensionDir `
    -BrowserExe $firefoxExe `
    -ExtUrl "about:addons"

# 6. Vivaldi
$vivaldiExe = @(
    "$env:LOCALAPPDATA\Vivaldi\Application\vivaldi.exe",
    "$env:ProgramFiles\Vivaldi\Application\vivaldi.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1

Register-BrowserExt -BrowserName "Vivaldi" `
    -RegKeyPath "HKCU:\Software\Vivaldi\Extensions\rapid_download_manager" `
    -ValueName "path" `
    -ValueData $ExtensionDir `
    -BrowserExe $vivaldiExe `
    -ExtUrl "vivaldi://extensions"

if (-not $Silent) {
    foreach ($r in $results) {
        $instStr = if ($r.Installed) { "[Detected]" } else { "[Not Installed]" }
        $color = if ($r.Installed) { "Green" } else { "DarkGray" }
        Write-Host " -> $($r.Browser): $instStr ($($r.Status))" -ForegroundColor $color
    }

    Write-Host ""
    Write-Host "✅ Registry keys successfully registered for all browsers!" -ForegroundColor Green
}

# Generate store-ready zip bundle if missing
$zipTarget = Join-Path $ScriptDir "dist\RapidExtension_StoreReady.zip"
try {
    $distDir = Split-Path -Parent $zipTarget
    if (-not (Test-Path $distDir)) { New-Item -Path $distDir -ItemType Directory -Force | Out-Null }
    Compress-Archive -Path "$ExtensionDir\*" -DestinationPath $zipTarget -Force
} catch {}

if ($LaunchBrowsers) {
    foreach ($r in $results) {
        if ($r.Installed -and $r.Exe) {
            try {
                Start-Process -FilePath $r.Exe -ArgumentList $r.ExtUrl
            } catch {}
        }
    }
}
