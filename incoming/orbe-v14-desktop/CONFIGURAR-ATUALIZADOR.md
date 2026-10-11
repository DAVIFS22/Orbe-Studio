# Configurar atualizações do Orbe v14.5 (GitHub Releases)

## 1. Configure o repositório
1. Crie um repositório GitHub e envie esta pasta inteira.
2. Em `src-tauri/tauri.conf.json`, substitua `SEU_USUARIO/SEU_REPOSITORIO` pelo dono e nome reais do repositório.
3. Não publique uma atualização assinada com a chave pública placeholder.

## 2. Gere as chaves de assinatura (uma única vez)
No terminal, na pasta `orbe-v14-desktop`, execute:

```powershell
npm install
npx tauri signer generate -w "$HOME/.tauri/orbe-updater.key"
```

Guarde a chave privada `.key` e a senha em local seguro. **Nunca envie a chave privada ao GitHub nem inclua no ZIP distribuído.** A chave pública gerada deve substituir `COLE_A_CHAVE_PUBLICA_GERADA_PELO_TAURI` em `src-tauri/tauri.conf.json`.

No GitHub, abra Settings → Secrets and variables → Actions e crie:
- `TAURI_SIGNING_PRIVATE_KEY`: conteúdo completo da chave privada.
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: senha da chave (pode ser vazio se a chave foi gerada sem senha).

## 3. Publicar uma atualização
Aumente a versão em `package.json`, `src-tauri/tauri.conf.json` e `src-tauri/Cargo.toml` para a mesma versão SemVer. Faça commit e envie uma tag, por exemplo:

```powershell
git add .
git commit -m "release: Orbe 14.5.1"
git tag v14.5.1
git push origin main --tags
```

O workflow `.github/workflows/release.yml` compila o instalador e publica os artefatos. Confirme na release que `latest.json`, o instalador e a assinatura `.sig` estão anexados.

## 4. Como funciona no aplicativo
Ao abrir o desktop, o Orbe verifica o endpoint GitHub Releases. Se houver versão mais recente, mostra uma caixa dentro do app com **Atualizar agora** e **Depois**. A instalação só começa após confirmação. A assinatura do pacote é validada pelo updater do Tauri.

## 5. Projetos recentes
Os projetos recentes do desktop são guardados em `%APPDATA%\br.orbe.imagemstudio\projects` (o caminho real é resolvido pelo Tauri). O aplicativo mantém até 20 entradas com os dados do projeto e miniatura. Não apaga imagens importadas originais. Os projetos antigos guardados somente no `localStorage` não são migrados automaticamente; abra e salve-os uma vez na versão desktop para passarem a usar o armazenamento persistente. Exporte também arquivos `.orbe.json` como backup.

## Importante
Este pacote contém a integração e os scripts, mas não pode publicar uma release sem o seu repositório GitHub e as chaves privadas. A compilação e o teste real do instalador devem ser feitos no Windows com Node, Rust e WebView2 instalados.
