#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{io::Write, path::PathBuf, process::{Command, Stdio}, fs};
use serde::{Deserialize, Serialize};
use tauri::{Manager, path::BaseDirectory};

fn find_script(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    if let Ok(path) = app.path().resolve("ai/remove_background.py", BaseDirectory::Resource) {
        if path.is_file() { return Ok(path); }
    }
    // Dev mode: the resource is in the project folder, not necessarily in the bundle.
    let cwd = std::env::current_dir().map_err(|e| format!("Não consegui ler a pasta atual: {e}"))?;
    for candidate in [cwd.join("ai/remove_background.py"), cwd.join("../ai/remove_background.py")] {
        if candidate.is_file() { return Ok(candidate); }
    }
    Err("Não encontrei ai/remove_background.py. Extraia o projeto completo e execute o app a partir da pasta orbe-v14-desktop.".into())
}

fn python_command() -> Result<Command, String> {
    if let Ok(path) = std::env::var("ORBE_PYTHON") {
        let mut c = Command::new(path);
        c.arg("--version");
        if c.output().map(|o| o.status.success()).unwrap_or(false) {
            let mut actual = Command::new(std::env::var("ORBE_PYTHON").unwrap());
            return Ok(actual);
        }
        return Err("ORBE_PYTHON aponta para um Python que não inicia. Corrija essa variável ou remova-a para detecção automática.".into());
    }
    // The Python launcher may have a different version installed. Try supported launchers
    // instead of failing immediately when Python 3.12 is not present.
    for (program, args) in [("py", vec!["-3.12"]), ("py", vec!["-3"]), ("python", vec![]), ("python3", vec![])] {
        let ok = Command::new(program).args(&args).arg("-c").arg("import sys; print(sys.version_info[:2])")
            .output().map(|o| o.status.success()).unwrap_or(false);
        if ok { let mut c = Command::new(program); c.args(args); return Ok(c); }
    }
    Err("Python não encontrado. Instale Python 3.12 (recomendado) ou defina ORBE_PYTHON com o caminho completo do python.exe.".into())
}

#[tauri::command]
fn remove_background_local(app: tauri::AppHandle, image_base64: String) -> Result<String, String> {
    if image_base64.len() > 120_000_000 {
        return Err("Imagem muito grande para o processamento em memória. Reduza a resolução e tente novamente.".into());
    }
    let script = find_script(&app)?;
    let mut command = python_command()?;
    let mut child = command.arg("-u").arg(script)
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()
        .map_err(|e| format!("Não consegui iniciar o Python local. Execute scripts\\instalar-ia-windows.bat e confirme que rembg/ONNX está instalado. Detalhe: {e}"))?;
    child.stdin.take().ok_or("Entrada do processador local indisponível")?
        .write_all(image_base64.as_bytes()).map_err(|e| format!("Falha ao enviar imagem ao modelo local: {e}"))?;
    let output = child.wait_with_output().map_err(|e| format!("Falha ao aguardar a IA local: {e}"))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr);
        return Err(format!("A IA local falhou. Execute scripts\\testar-ia-local.bat para diagnosticar. {}", detail.chars().take(1800).collect::<String>()));
    }
    String::from_utf8(output.stdout).map_err(|e| format!("Resposta inválida do modelo local: {e}"))
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct RecentProject { id: String, name: String, width: u32, height: u32, thumbnail: String, updated_at: u64 }
fn recent_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| format!("Pasta de dados do Orbe indisponível: {e}"))?.join("projects");
    fs::create_dir_all(&dir).map_err(|e| format!("Não consegui criar a pasta de projetos: {e}"))?;
    Ok(dir)
}
#[tauri::command]
fn save_recent_project(app: tauri::AppHandle, id: String, name: String, width: u32, height: u32, thumbnail: String, project_json: String) -> Result<(), String> {
    if project_json.len() > 100_000_000 { return Err("Projeto excede o limite de 100 MB para o índice de recentes.".into()); }
    let dir = recent_dir(&app)?;
    let safe_id: String = id.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').collect();
    if safe_id.is_empty() { return Err("ID de projeto inválido.".into()); }
    fs::write(dir.join(format!("{safe_id}.orbe.json")), project_json).map_err(|e| format!("Não consegui salvar o projeto local: {e}"))?;
    let recent = RecentProject { id: safe_id.clone(), name, width, height, thumbnail, updated_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs() };
    fs::write(dir.join(format!("{safe_id}.meta.json")), serde_json::to_vec(&recent).map_err(|e| e.to_string())?).map_err(|e| format!("Não consegui salvar os dados do projeto: {e}"))?;
    let mut items = read_recents(&dir);
    items.retain(|x| x.id != safe_id);
    items.push(recent);
    items.sort_by(|a,b| b.updated_at.cmp(&a.updated_at));
    for old in items.iter().skip(20) { let _ = fs::remove_file(dir.join(format!("{}.orbe.json", old.id))); let _ = fs::remove_file(dir.join(format!("{}.meta.json", old.id))); }
    Ok(())
}
fn read_recents(dir: &PathBuf) -> Vec<RecentProject> {
    let mut items = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) { for entry in entries.flatten() { let p=entry.path(); if p.extension().and_then(|x|x.to_str()) == Some("json") && p.file_name().and_then(|x|x.to_str()).map(|x|x.ends_with(".meta.json")).unwrap_or(false) { if let Ok(bytes)=fs::read(p) { if let Ok(item)=serde_json::from_slice::<RecentProject>(&bytes) { items.push(item); } } } } }
    items.sort_by(|a,b| b.updated_at.cmp(&a.updated_at)); items
}
#[tauri::command]
fn list_recent_projects(app: tauri::AppHandle) -> Result<Vec<RecentProject>, String> { Ok(read_recents(&recent_dir(&app)?).into_iter().take(20).collect()) }
#[tauri::command]
fn load_recent_project(app: tauri::AppHandle, id: String) -> Result<String, String> {
    let safe_id: String = id.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').collect();
    fs::read_to_string(recent_dir(&app)?.join(format!("{safe_id}.orbe.json"))).map_err(|e| format!("Não consegui abrir este projeto recente: {e}"))
}
#[tauri::command]
fn remove_recent_project(app: tauri::AppHandle, id: String) -> Result<(), String> {
    let safe_id: String = id.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').collect();
    let dir=recent_dir(&app)?; let _=fs::remove_file(dir.join(format!("{safe_id}.orbe.json"))); let _=fs::remove_file(dir.join(format!("{safe_id}.meta.json"))); Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![remove_background_local, save_recent_project, list_recent_projects, load_recent_project, remove_recent_project])
        .run(tauri::generate_context!())
        .expect("falha ao iniciar Orbe Imagem Studio");
}
