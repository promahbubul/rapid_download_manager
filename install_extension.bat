@echo off
title Rapid Download Manager - Auto Browser Integration
color 0B
cd /d "%~dp0"

echo =========================================================
echo    ⚡ Rapid Download Manager — Auto Browser Integration
echo =========================================================
echo.
echo Detecting installed browsers and registering Registry keys...
echo.

powershell.exe -ExecutionPolicy Bypass -NoProfile -File "%~dp0auto_integrate_browsers.ps1"

echo.
echo =========================================================
echo  [1] Open Extension Manager in all detected browsers
echo  [2] Open Extension Folder in Windows Explorer
echo  [3] Exit
echo =========================================================
set /p choice="Select an option (1-3) [Default 1]: "

if "%choice%"=="" set choice=1
if "%choice%"=="1" (
    powershell.exe -ExecutionPolicy Bypass -NoProfile -File "%~dp0auto_integrate_browsers.ps1" -LaunchBrowsers
)
if "%choice%"=="2" (
    explorer.exe "%~dp0extension"
)

echo.
echo Done! Press any key to exit.
pause >nul
