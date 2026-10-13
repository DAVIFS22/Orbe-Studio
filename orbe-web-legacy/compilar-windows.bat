@echo off
setlocal
cd /d "%~dp0"
if not exist package.json (echo ERRO: execute dentro da pasta que contem package.json.& pause & exit /b 1)
if not exist node_modules (call npm install || goto erro)
call npm run tauri build
if errorlevel 1 goto erro
echo Compilacao concluida. Procure o instalador em src-tauri\target\release\bundle.
pause
exit /b 0
:erro
echo Compilacao falhou. Veja a primeira mensagem de erro acima.
pause
exit /b 2
