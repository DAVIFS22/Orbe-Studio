# ORBE 14.8.7 — revisão de identidade e código

Esta revisão substitui referências de marca ORBE/ORBE nos arquivos de código, configuração, documentação operacional e scripts, e renomeia arquivos de marca correspondentes. O nome distribuído do aplicativo é ORBE.

## Preservado intencionalmente

Avisos legais, licenças, NOTICE e atribuições de autoria foram preservados quando mencionam os projetos upstream. Eles não são elementos da interface e não devem ser removidos para ocultar a origem/licença do código reutilizado.

## Validação

- Arquivos textuais processados: 1163 alterados; ocorrências substituídas: 10779.
- Este pacote ainda exige compilação no Windows. A substituição de nomes em um workspace Rust amplo pode revelar dependências de nomes/caminhos não detectadas por revisão estática; não declare como compilação aprovada até executar `cargo check --workspace` e `cargo build --release` no Windows.
- Se algum código serializado/protocolo externo depender de identificadores antigos, valide compatibilidade antes de migrar dados existentes.


## Segunda varredura

Varredura sem diferenciar maiúsculas/minúsculas: 337 arquivos textuais adicionais/atualizados; 3647 ocorrências substituídas fora dos arquivos de licença/atribuição.
