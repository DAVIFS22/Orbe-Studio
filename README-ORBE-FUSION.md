# Orbe Studio 14.8.2

Editor de imagens nativo do Orbe Studio, construído com Rust, egui/eframe e wgpu. O pacote reúne a aplicação nativa, os componentes do motor de edição e o projeto web/Tauri anterior do Orbe para preservar referências e funcionalidades que ainda precisam ser portadas.

## Compilar no Windows

1. Instale Rust 1.95 ou superior via rustup e Visual Studio Build Tools com C++ para desktop.
2. Extraia o ZIP completo em uma pasta gravável.
3. Execute `compilar-orbe-studio-windows.bat`.
4. Depois de uma compilação bem-sucedida, execute `executar-orbe-studio-windows.bat` ou `target\release\orbe-studio.exe`.

O primeiro build pode baixar dependências do Cargo e demorar. Esta distribuição ainda precisa ser compilada e testada no Windows antes de ser considerada validada.

## Conteúdo

- `apps/` e `crates/`: aplicação nativa e motor de edição.
- `orbe-web-legacy/`: código anterior do Orbe web/Tauri, mantido para referência e para não descartar funcionalidades existentes; ainda é necessário portar e validar suas funções na interface nativa.
- `ATTRIBUTION.md`, `NOTICE`, `LICENSE-MIT`, `LICENSE-APACHE`: avisos legais e atribuições dos componentes de terceiros. Eles são mantidos para cumprir as licenças aplicáveis.

## Importante

A interface e a identidade visível foram identificadas como Orbe Studio. Alguns nomes internos de crates permanecem por compatibilidade técnica; renomeá-los indiscriminadamente poderia quebrar dependências e testes. Os avisos de autoria/licença de terceiros não foram removidos. A integração completa de cada função precisa ser confirmada por compilação e testes funcionais.
