# Orbe Studio 14.8.4 — revisão de identidade e links

## Alterações nesta revisão
- Removido o botão Discord da barra superior, da tela inicial e da janela Sobre.
- Removidas referências visíveis a Discord no fluxo principal de interface.
- Atualizados os links de suporte do instalador para o repositório e a página de issues do Orbe Studio.
- Atualizados o título e a mensagem de carregamento da versão web para Orbe Studio.
- Atualizados alguns textos públicos do instalador para Orbe Studio.

## Limites e verificações
- Nomes internos de crates, módulos, variáveis de ambiente, protocolos e formatos de arquivo foram preservados quando sua alteração poderia quebrar compatibilidade ou compilação.
- Avisos de licença, autoria e atribuição foram preservados; não são elementos promocionais da interface.
- Este pacote foi revisado estaticamente. Não foi possível executar `cargo check`, testes ou gerar o instalador Windows neste ambiente.
- O usuário deve compilar a partir de uma pasta nova e reportar o primeiro erro completo, caso apareça.
