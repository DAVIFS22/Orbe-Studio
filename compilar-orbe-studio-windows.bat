@echo off
setlocal
cd /d "%~dp0"

title ORBE Studio 14.8.9 - Compilacao Nativa

echo ==============================================
echo       ORBE STUDIO 14.8.9
echo       COMPILACAO NATIVA PARA WINDOWS
echo ==============================================
echo.

where cargo >nul 2>nul
if errorlevel 1 (
    echo ERRO: Rust/Cargo nao encontrado.
    echo.
    echo Instale o Rust pelo site:
    echo https://rustup.rs/
    echo.
    echo Tambem instale o Visual Studio Build Tools
    echo com os componentes C++ para desktop.
    echo.
    pause
    exit /b 1
)

if not exist Cargo.toml (
    echo ERRO: Cargo.toml nao encontrado.
    echo Extraia o projeto completo antes de compilar.
    echo.
    pause
    exit /b 1
)

if not exist apps\orbe-studio\Cargo.toml (
    echo ERRO: projeto principal nao encontrado.
    echo Verifique a pasta apps\orbe-studio.
    echo.
    pause
    exit /b 1
)

if not exist apps\orbe-studio\src\main.rs (
    echo ERRO: codigo-fonte principal nao encontrado.
    echo.
    pause
    exit /b 1
)

echo [1/2] Compilando o ORBE Studio nativo...
echo.
echo Recursos habilitados: HEIF
echo Aguarde. A primeira compilacao pode demorar.
echo.

cargo build --release -p orbe --bin orbe-studio --features heif

if errorlevel 1 (
    echo.
    echo ==============================================
    echo ERRO: A COMPILACAO FALHOU
    echo ==============================================
    echo.
    echo Confira as mensagens de erro do Cargo.
    pause
    exit /b 1
)

if not exist target\release\orbe-studio.exe (
    echo.
    echo ERRO: O executavel nao foi encontrado apos a compilacao.
    pause
    exit /b 1
)

echo.
echo [2/2] Compilacao concluida com sucesso!
echo.
echo Executavel:
echo %CD%\target\release\orbe-studio.exe
echo.

for %%F in ("target\release\orbe-studio.exe") do (
    echo Tamanho: %%~zF bytes
)

echo.
echo Deseja iniciar o ORBE Studio agora?
choice /C SN /N /M "Digite S para iniciar ou N para sair: "

if errorlevel 2 goto fim

if errorlevel 1 (
    start "" "%CD%\target\release\orbe-studio.exe"
)

:fim
echo.
echo Processo concluido.
pause
endlocal