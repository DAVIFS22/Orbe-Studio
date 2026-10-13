# Relatório de revisão — Orbe Studio 14.8.0 (histórico)

## Verificações executadas

- TOML OK: Cargo.toml
- TOML OK: xtask/Cargo.toml
- TOML OK: apps/orbe-studio/Cargo.toml
- TOML OK: apps/orbe-studio-cli/Cargo.toml
- TOML OK: apps/orbe-studio-web/Cargo.toml
- TOML OK: orbe-web-legacy/src-tauri/Cargo.toml
- TOML OK: crates/affinity/Cargo.toml
- TOML OK: crates/engine/Cargo.toml
- TOML OK: crates/vector/Cargo.toml
- TOML OK: crates/ui-egui/Cargo.toml
- TOML OK: crates/psd/Cargo.toml
- TOML OK: crates/testkit/Cargo.toml
- TOML OK: crates/paint/Cargo.toml
- TOML OK: crates/plugins/Cargo.toml
- TOML OK: crates/codecs/Cargo.toml
- TOML OK: crates/raster/Cargo.toml
- TOML OK: crates/tablet/Cargo.toml
- TOML OK: crates/color/Cargo.toml
- TOML OK: crates/io/Cargo.toml
- TOML OK: crates/compose/Cargo.toml
- TOML OK: crates/format/Cargo.toml
- TOML OK: crates/text/Cargo.toml
- TOML OK: crates/algo/Cargo.toml
- TOML OK: crates/heif/Cargo.toml
- TOML OK: crates/cms/Cargo.toml
- TOML OK: crates/doc/Cargo.toml
- TOML OK: crates/automation/Cargo.toml
- TOML OK: crates/ops/Cargo.toml
- TOML OK: crates/gpu/Cargo.toml
- TOML OK: crates/raw/Cargo.toml
- TOML OK: crates/geom/Cargo.toml
- TOML OK: crates/raw/fuzz/Cargo.toml
- TOML OK: crates/format/fuzz/Cargo.toml
- TOML OK: crates/codecs/fuzz/Cargo.toml
- TOML OK: crates/psd/fuzz/Cargo.toml
- TOML OK: crates/affinity/fuzz/Cargo.toml
- TOML OK: examples/plugins/invert-rs/Cargo.toml
- JSON OK: orbe-web-legacy/package.json
- Image OK: assets/app-icon/orbe-1024.png
- Image OK: assets/app-icon/hicolor/256x256/apps/orbe-studio.png
- Image OK: assets/app-icon/orbe-studio.ico

## Limitações e pendências

- Rust compilation/tests not run: rustc/cargo are not installed in this environment.
- Orbe web/Tauri-specific features are preserved in orbe-web-legacy, not yet ported into the native egui interface.
- The native app still uses ORBE engine/crate names and upstream service links intentionally; this preserves source attribution and internal API compatibility.

## Resultado

A estrutura do pacote e os manifests TOML/JSON foram verificados estaticamente. Isso não substitui `cargo check`, `cargo test`, build Windows nem testes funcionais de abrir/editar/salvar documentos. A integração total de recursos específicos do antigo Orbe web ainda não está comprovada.
