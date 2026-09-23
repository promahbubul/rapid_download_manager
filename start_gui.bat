@echo off
taskkill /F /IM rapid-gui.exe 2>nul
start "" "%~dp0rapid-gui.exe"
