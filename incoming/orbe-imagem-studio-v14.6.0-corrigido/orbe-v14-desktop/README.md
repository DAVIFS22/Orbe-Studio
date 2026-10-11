# Orbe Imagem Studio v14.4 — Tauri + IA local U2Net

## Objetivo
Executar remoção de fundo localmente no Windows usando Tauri/Rust, Python 3.12, `rembg` e U2Net via ONNX Runtime CPU. Esse fluxo não envia as imagens a uma API de nuvem.

## Requisitos
- Windows 10/11 64-bit
- Node.js LTS
- Rust via rustup
- Microsoft C++ Build Tools e WebView2 Runtime
- Python 3.12 recomendado; os scripts também tentam localizar outros runtimes Python instalados
- Internet e espaço em disco para instalar dependências e baixar o modelo na primeira execução

## Instalar e provar a inferência
1. Instale os requisitos acima.
2. Extraia este ZIP para uma pasta gravável, por exemplo `C:\OrbeStudio`.
3. Execute `scripts\instalar-ia-windows.bat`. O script tenta Python 3.12, depois o runtime padrão do launcher `py -3` e por fim `python`; instala as dependências nesse runtime e executa um teste de inferência real.
4. Espere o primeiro download do U2Net terminar. O processo pode levar alguns minutos e baixar centenas de MB.
5. Confirme o JSON com `"status": "PASS"` e `"inference_executed": true`. Isso comprova que o modelo carregou, executou e produziu PNG com canal alfa; não garante bordas perfeitas para todas as imagens.
6. Execute `scripts\testar-ia-local.bat` novamente sempre que precisar revalidar.

## Rodar o app desktop
1. Extraia o ZIP inteiro. Entre na pasta `orbe-v14-desktop` (é nela que fica `package.json`).
2. Mais simples: dê duplo clique em `iniciar-desenvolvimento.bat`. Ele muda para a pasta correta, instala dependências se necessário e inicia o Tauri.
3. Manualmente, abra o CMD na pasta `orbe-v14-desktop` e execute `npm install` e depois `npm run tauri dev`. Não execute esses comandos em `C:\` nem dentro de `scripts`.
4. Importe uma fotografia, selecione a camada raster e acione a ferramenta **Remover fundo**.
5. Confira o resultado sobre o quadriculado, especialmente cabelo e bordas. Teste desfazer/refazer para validar o histórico.
6. Para gerar um instalador Windows, execute `npm run tauri build`.

## Diagnóstico
- `ModuleNotFoundError: rembg`: execute `scripts\instalar-ia-windows.bat` com Python 3.12 instalado.
- Erro de download/modelo: confirme conexão, espaço em disco e tente novamente.
- Erro ao iniciar o Python no app: o app tenta `py -3.12`, `py -3`, `python` e `python3`; também aceita `ORBE_PYTHON` com o caminho completo do `python.exe`. Se precisar usar um caminho específico, defina `ORBE_PYTHON` para o caminho completo do `python.exe`.
- `PASS` no teste não substitui teste visual com fotografias reais nem a compilação final do Tauri.

## Limitações atuais
- U2Net é segmentação geral, não inpainting generativo nem super-resolução.
- A ponte transfere PNG em base64 entre WebView e Python; imagens muito grandes podem consumir muita RAM.
- O app desktop e o instalador ainda precisam ser compilados e testados no Windows de destino. Este pacote não inclui Python, dependências instaladas nem pesos do modelo.
- Grupos são lógicos; vínculo de máscara independente e healing generativo ainda não estão implementados.


## Atualizador v14.5 e projetos recentes
Leia `CONFIGURAR-ATUALIZADOR.md` para configurar GitHub Releases, assinatura e publicação. O updater é opcional até configurar a chave pública e o repositório real. Os recentes do desktop passam a ser persistidos na pasta de dados do aplicativo.

## Versão 14.6.0 — preparação

Veja `CHANGELOG-14.6.0.md` para alterações, verificações e pendências do atualizador. Antes de compilar uma release assinada, configure a chave pública real do atualizador em `src-tauri/tauri.conf.json` e mantenha a chave privada apenas no armazenamento seguro do GitHub Actions/ambiente local protegido.
