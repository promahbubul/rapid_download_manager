@echo off
title Rapid Download Manager - Browser Extension Installer
color 0B
cd /d "%~dp0"

echo =========================================================
echo    ⚡ Rapid Download Manager — Browser Extension Setup
echo =========================================================
echo.
echo Registering Rapid Download Manager extension in Windows Registry...

reg add "HKCU\Software\Google\Chrome\Extensions\rapid_download_manager" /v "path" /t REG_SZ /d "%~dp0extension" /f >nul 2>&1
reg add "HKCU\Software\Microsoft\Edge\Extensions\rapid_download_manager" /v "path" /t REG_SZ /d "%~dp0extension" /f >nul 2>&1
reg add "HKCU\Software\BraveSoftware\Brave-Browser\Extensions\rapid_download_manager" /v "path" /t REG_SZ /d "%~dp0extension" /f >nul 2>&1

echo [OK] Registry keys registered for Chrome, Edge, and Brave!
echo.
echo =========================================================
echo To complete 1-click install in your browser:
echo 1. Open your browser:
echo    - Chrome: chrome://extensions
echo    - Edge:   edge://extensions
echo    - Brave:  brave://extensions
echo 2. Enable "Developer mode" toggle (top-right).
echo 3. Click "Load unpacked" and select this folder:
echo    "%~dp0extension"
echo =========================================================
echo.
echo Opening the extension folder for you...
explorer.exe "%~dp0extension"
pause
