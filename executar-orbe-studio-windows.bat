@echo off
setlocal
cd /d "%~dp0"
if exist target\release\orbe-studio.exe (
  start "" target\release\orbe-studio.exe
) else (
  echo O aplicativo ainda nao foi compilado.
  echo Execute compilar-orbe-studio-windows.bat primeiro.
  pause
)
