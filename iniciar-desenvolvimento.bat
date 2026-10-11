@echo off
setlocal
cd /d "%~dp0"
echo Pasta do projeto: %CD%
if not exist package.json (
  echo ERRO: package.json nao encontrado. Extraia o ZIP inteiro e execute este arquivo dentro da pasta orbe-v14-desktop.
  pause
  exit /b 1
)
where node >nul 2>nul || (echo ERRO: instale Node.js LTS. & pause & exit /b 2)
where npm >nul 2>nul || (echo ERRO: npm nao encontrado. Reinstale Node.js LTS e abra um terminal novo. & pause & exit /b 2)
if not exist node_modules (
  echo Instalando dependencias npm na pasta correta...
  call npm install
  if errorlevel 1 goto erro
)
echo Iniciando Orbe Imagem Studio via Tauri...
call npm run tauri dev
if errorlevel 1 goto erro
exit /b 0
:erro
echo.
echo O inicio falhou. Confira os requisitos do README.md e o erro acima.
pause
exit /b 3
