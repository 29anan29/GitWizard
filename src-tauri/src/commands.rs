use crate::{commit as commit_mod, config as app_config, events, push, repo, stage};
use std::path::Path;
use std::sync::Arc;
use tauri::AppHandle;

use crate::config::AppConfig;

#[tauri::command]
pub async fn open_repo_dialog(title: String) -> Result<Option<String>, String> {
    tokio::task::spawn_blocking(move || {
        rfd::FileDialog::new()
            .set_title(&title)
            .pick_folder()
            .map(|p| p.to_string_lossy().into_owned())
    })
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn validate_repo(app: AppHandle, repo_path: String) -> Result<repo::RepoInfo, String> {
    events::cmd(
        &app,
        &format!("git -C {} rev-parse --abbrev-ref HEAD", shell_quote(&repo_path)),
    );
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        repo::info(&r)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_status(
    app: AppHandle,
    repo_path: String,
) -> Result<Vec<repo::FileEntry>, String> {
    events::cmd(
        &app,
        &format!("git -C {} status --porcelain", shell_quote(&repo_path)),
    );
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        repo::status_entries(&r)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn stage_files(
    app: AppHandle,
    repo_path: String,
    files: Vec<String>,
) -> Result<(), String> {
    events::cmd(&app, &format!("git add {}", summarize_paths(&files)));
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        stage::stage(&r, &files)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn unstage_files(
    app: AppHandle,
    repo_path: String,
    files: Vec<String>,
) -> Result<(), String> {
    events::cmd(&app, &format!("git restore --staged {}", summarize_paths(&files)));
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        stage::unstage(&r, &files)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn staged_summary(
    repo_path: String,
    files: Vec<String>,
) -> Result<stage::StagedSummary, String> {
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        stage::summary(&r, &files)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn commit_repo(
    app: AppHandle,
    repo_path: String,
    message: String,
    identity_user: Option<String>,
    identity_email: Option<String>,
) -> Result<String, String> {
    let quoted = format!("\"{}\"", message.replace('"', "\\\""));
    events::cmd(&app, &format!("git commit -m {quoted}"));
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        let oid = commit_mod::commit(
            &r,
            &message,
            identity_user.as_deref().zip(identity_email.as_deref()),
        )?;
        let first_line = message.lines().next().unwrap_or("").to_string();
        events::out(&app, &format!("[{}] {}", &oid[..7.min(oid.len())], first_line));
        Ok(oid)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn push_remote(
    app: AppHandle,
    repo_path: String,
    remote_name: String,
    branch: String,
    username: Option<String>,
    password: Option<String>,
) -> Result<(), String> {
    events::cmd(&app, &format!("git push {remote_name} {branch}"));
    let app_out = app.clone();
    let app_prog = app.clone();
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        let prog_app = app_prog.clone();
        let prog: Arc<dyn Fn(usize, usize) + Send + Sync> =
            Arc::new(move |c, t| events::progress(&prog_app, c, t));
        let sb: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |line: &str| events::out(&app_out, line));
        push::push(&r, &remote_name, &branch, username, password, prog, sb)?;
        events::progress(&app_prog, 1, 1);
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn get_config() -> AppConfig {
    app_config::load()
}

#[tauri::command]
pub fn save_config(config: AppConfig) -> Result<(), String> {
    let mut cfg = config;
    if cfg.locale != "zh-CN" && cfg.locale != "en" {
        cfg.locale = "zh-CN".into();
    }
    app_config::save(&cfg)
}

fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

fn summarize_paths(files: &[String]) -> String {
    const MAX: usize = 5;
    let shown: Vec<&str> = files.iter().take(MAX).map(|s| s.as_str()).collect();
    let mut line = shown.join(" ");
    if files.len() > MAX {
        line.push_str(&format!(" (+{} 个文件)", files.len() - MAX));
    }
    line
}
