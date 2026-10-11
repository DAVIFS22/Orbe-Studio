@echo off
setlocal EnableExtensions
cd /d "%~dp0.."
set "PY="
where py >nul 2>nul
if not errorlevel 1 (
  py -3.12 --version >nul 2>nul
  if not errorlevel 1 set "PY=py -3.12"
  if not defined PY (
    py -3 --version >nul 2>nul
    if not errorlevel 1 set "PY=py -3"
  )
)
if not defined PY (
  python --version >nul 2>nul
  if not errorlevel 1 set "PY=python"
)
if not defined PY goto nopython
echo Usando runtime: %PY%
%PY% --version || goto erro
%PY% -m pip install --upgrade pip
if errorlevel 1 goto erro
%PY% -m pip install -r "%~dp0..\ai\requirements-ai.txt"
if errorlevel 1 goto erro
echo.
echo Dependencias instaladas. Agora vou executar uma inferencia U2Net real.
echo Na primeira execucao o modelo pode baixar centenas de MB.
%PY% ai\remove_background.py --self-test
if errorlevel 1 goto erro
echo.
echo SUCESSO: inferencia U2Net executada. Consulte o JSON acima.
pause
exit /b 0
:nopython
echo ERRO: nao encontrei Python. Instale Python 3.12 ou execute py install 3.12.
echo Se o Python ja estiver instalado, tente: py -3 --version
pause
exit /b 1
:erro
echo.
echo FALHA: instalacao ou inferencia nao concluida.
echo Confira a conexao, espaco em disco e a mensagem de erro acima.
pause
exit /b 2
