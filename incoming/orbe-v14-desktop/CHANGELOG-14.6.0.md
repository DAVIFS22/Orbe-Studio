# Orbe Imagem Studio 14.6.0 — pacote de atualização do projeto

## Alterações preparadas

- Atualizados os metadados de versão para 14.6.0.
- Tela inicial reorganizada com atalhos para novo projeto, abrir imagem, importar camada, exportação/download, configurações e recursos de IA.
- Adicionado painel de configurações agrupado por aparência, exportação, fluxo de edição e ajuda.
- Preferências de tema, movimento, formato de exportação, qualidade, escala, tamanho inicial do pincel, confirmação de exclusão e painel de camadas são persistidas no perfil local do app.
- A ferramenta Texto agora seleciona texto existente no clique simples; a edição continua disponível por duplo clique, reduzindo entradas acidentais no modo de edição.
- Exportação/download com PNG, JPEG e WebP, escala e qualidade; JPEG usa fundo branco quando transparência é escolhida, pois JPEG não suporta canal alfa.
- Corrigido o cálculo de progresso do atualizador para acumular os bytes baixados, em vez de tratar o tamanho de um único bloco como progresso total.
- Endpoint de releases apontado para o repositório DAVIFS22/orbe-imagem-studio.

## Validação feita neste pacote

- `node --check` passou para o JavaScript do atualizador e para os dois blocos de JavaScript embutidos no HTML.
- O pacote foi inspecionado e os arquivos de versão foram atualizados.

## Pendências antes de gerar/publicar instalador

- `src-tauri/tauri.conf.json` neste ZIP continha o texto de exemplo `COLE_A_CHAVE_PUBLICA_GERADA_PELO_TAURI` no campo `plugins.updater.pubkey`. O build de release/atualizador não deve ser considerado pronto até substituir isso pela chave pública Minisign real correspondente à chave privada de assinatura configurada no GitHub Actions.
- A qualidade do removedor de fundo U2Net depende do modelo e da imagem de entrada; esta edição não substitui U2Net por um novo modelo de segmentação nem afirma melhoria comprovada de recorte de cabelo sem testes visuais com fotos reais.
- Não foi gerado instalador Windows nem testada instalação/atualização ponta a ponta neste ambiente.
