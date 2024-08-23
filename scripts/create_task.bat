@echo off

set SCRIPT_DIR=%~dp0
set PROGRAM_PATH=%SCRIPT_DIR%..\target\release\duck_updater.exe

schtasks /create /tn "DuckUpdater" /tr "cmd /c cd /d %SCRIPT_DIR%.. && %PROGRAM_PATH% 2>&1" /sc minute /mo 15 /f

echo Task created successfully.