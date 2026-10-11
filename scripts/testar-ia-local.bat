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
if not defined PY (
  echo ERRO: nao encontrei Python. Instale Python 3.12 ou execute py install 3.12.
  pause
  exit /b 1
)
echo Runtime: %PY%
%PY% --version
%PY% -c "import rembg, onnxruntime, PIL; print('rembg',getattr(rembg,'__version__','unknown')); print('onnxruntime',onnxruntime.__version__); print('providers',onnxruntime.get_available_providers())" || (echo ERRO: dependencias ausentes. Execute scripts\instalar-ia-windows.bat. & pause & exit /b 2)
echo.
echo Executando inferencia real U2Net. A primeira vez pode baixar o modelo.
%PY% ai\remove_background.py --self-test
if errorlevel 1 (echo. & echo FALHA: inferencia nao concluida. Copie o JSON de erro para diagnostico. & pause & exit /b 3)
echo.
echo PASSOU: o modelo carregou, executou inferencia e devolveu PNG com canal alfa.
pause
