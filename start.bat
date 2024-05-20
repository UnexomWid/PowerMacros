@echo OFF

net session >nul 2>&1
if %errorlevel% == 0 (
    goto :run
)

echo PowerMacros needs to be ran as admin, requesting permissions...
powershell start cmd.exe -verb runas -ArgumentList '/c', 'cd /d "%cd%" ^&^& "%0"' & exit /b

:run

echo Starting AHK script...

START uwpm.ahk

echo Starting PowerMacros...
cargo run --release