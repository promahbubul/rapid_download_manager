@echo off
taskkill /F /IM rapid-gui.exe 2>nul
start "" "%~dp0target\x86_64-pc-windows-gnullvm\debug\rapid-gui.exe"
