# ORBE Studio 14.8.9

Editor de imagens nativo para Windows, desenvolvido em Rust com egui, eframe e wgpu.

O projeto contém a aplicação nativa, os componentes do motor de edição e o código legado do Orbe Web/Tauri, mantido para referência e para facilitar a migração de funcionalidades.

## Requisitos

- Windows 10 ou Windows 11 de 64 bits.
- Rust 1.95 ou superior, instalado por meio do rustup.
- Visual Studio Build Tools com os componentes de desenvolvimento C++ para desktop.

## Compilar no Windows

1. Extraia o projeto completo em uma pasta com permissão de escrita.
2. Abra o CMD nessa pasta.
3. Execute:

   ```cmd
   cargo build --release -p orbe --bin orbe-studio --features heif