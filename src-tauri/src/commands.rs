use crate::{
    branch as branch_mod, commit as commit_mod, config as app_config, credentials, events,
    ignore as ignore_mod, log as log_mod, merge, pull, push, repo, reset, ssh, stage, undo,
};
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
    target_branch: Option<String>,
    username: Option<String>,
    password: Option<String>,
) -> Result<(), String> {
    let dst = target_branch.clone().unwrap_or_else(|| branch.clone());
    let log_line = if dst == branch {
        format!("git push {remote_name} {branch}")
    } else {
        format!("git push {remote_name} refs/heads/{branch}:refs/heads/{dst}")
    };
    events::cmd(&app, &log_line);
    let app_out = app.clone();
    let app_prog = app.clone();
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        let prog_app = app_prog.clone();
        let prog: Arc<dyn Fn(usize, usize) + Send + Sync> =
            Arc::new(move |c, t| events::progress(&prog_app, c, t));
        let sb: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |line: &str| events::out(&app_out, line));
        push::push(&r, &remote_name, &branch, &dst, username, password, prog, sb)?;
        events::progress(&app_prog, 1, 1);
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn pull_branch(
    app: AppHandle,
    repo_path: String,
    remote_name: String,
    branch: String,
    username: Option<String>,
    password: Option<String>,
) -> Result<pull::PullOutcome, String> {
    events::cmd(
        &app,
        &format!("git pull --ff-only {remote_name} {branch}"),
    );
    let app_out = app.clone();
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        let identity = app_config::load();
        let sb: Arc<dyn Fn(&str) + Send + Sync> =
            Arc::new(move |line: &str| events::out(&app_out, line));
        pull::pull(
            &r,
            &remote_name,
            &branch,
            username,
            password,
            identity.user_name.as_deref().zip(identity.user_email.as_deref()),
            sb,
        )
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

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub current: String,
    pub latest_tag: Option<String>,
    pub release_url: String,
    pub available: bool,
}

fn version_triplet(v: &str) -> (u64, u64, u64) {
    let mut it = v.trim().split('.').map(|part| {
        part.chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap_or(0)
    });
    (
        it.next().unwrap_or(0),
        it.next().unwrap_or(0),
        it.next().unwrap_or(0),
    )
}

#[tauri::command]
pub async fn check_updates(proxy: Option<String>) -> Result<UpdateInfo, String> {
    tokio::task::spawn_blocking(move || {
        const CURRENT: &str = env!("CARGO_PKG_VERSION");
        let mut builder = ureq::AgentBuilder::new().timeout(std::time::Duration::from_secs(12));
        if let Some(p) = proxy.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            let pr = ureq::Proxy::new(p).map_err(|e| format!("PROXY:{e}"))?;
            builder = builder.proxy(pr);
        }
        let agent = builder.build();
        let resp = agent
            .get("https://api.github.com/repos/29anan29/GitWizard/releases/latest")
            .set("User-Agent", "gitwizard-update-check")
            .set("Accept", "application/vnd.github+json")
            .call()
            .map_err(|e| match e {
                ureq::Error::Status(code, _) => format!("HTTP:{code}"),
                other => format!("NET:{other}"),
            })?;
        let body = resp.into_string().map_err(|e| format!("READ:{e}"))?;
        let v: serde_json::Value =
            serde_json::from_str(&body).map_err(|e| format!("PARSE:{e}"))?;
        let tag = v
            .get("tag_name")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .trim_start_matches('v')
            .to_string();
        let url = v
            .get("html_url")
            .and_then(|x| x.as_str())
            .unwrap_or("https://github.com/29anan29/GitWizard/releases")
            .to_string();

        let cur = version_triplet(CURRENT);
        let lat = version_triplet(&tag);
        Ok(UpdateInfo {
            current: CURRENT.to_string(),
            latest_tag: if tag.is_empty() { None } else { Some(tag) },
            release_url: url,
            available: lat > cur,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn open_external(url: String) -> Result<(), String> {
    if !url.starts_with("https://") {
        return Err("仅允许打开 https 链接".to_string());
    }
    open::that(url).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_credential(username: String, password: String) -> Result<(), String> {
    credentials::save(username, password)
}

#[tauri::command]
pub fn load_credential(username: String) -> Result<Option<String>, String> {
    credentials::load(username)
}

#[tauri::command]
pub fn delete_credential(username: String) -> Result<(), String> {
    credentials::delete(username)
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct BranchList {
    pub local: Vec<String>,
    pub remote: Vec<String>,
}

#[tauri::command]
pub async fn list_branches(repo_path: String) -> Result<BranchList, String> {
    use git2::BranchType;
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        let mut local: Vec<String> = r
            .branches(Some(BranchType::Local))
            .map_err(err_msg)?
            .filter_map(|b| b.ok())
            .map(|(b, _)| b.name().unwrap_or(None).unwrap_or("").to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let mut remote: Vec<String> = r
            .branches(Some(BranchType::Remote))
            .map_err(err_msg)?
            .filter_map(|b| b.ok())
            .map(|(b, _)| b.name().unwrap_or(None).unwrap_or("").to_string())
            .filter(|s| s.starts_with("origin/") && !s.ends_with("HEAD"))
            .map(|s| s.trim_start_matches("origin/").to_string())
            .collect();
        local.sort();
        local.dedup();
        remote.sort();
        remote.dedup();
        Ok(BranchList { local, remote })
    })
    .await
    .map_err(|e| e.to_string())?
}

fn err_msg(e: git2::Error) -> String {
    e.message().to_string()
}

#[tauri::command]
pub async fn create_branch(
    app: AppHandle,
    repo_path: String,
    name: String,
    switch: bool,
) -> Result<(), String> {
    events::cmd(
        &app,
        &format!(
            "git branch {name}{}",
            if switch { " && git checkout <name>" } else { "" }
        ),
    );
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        branch_mod::create(&r, name.trim(), switch)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn checkout_branch(
    app: AppHandle,
    repo_path: String,
    name: String,
) -> Result<(), String> {
    events::cmd(&app, &format!("git checkout {name}"));
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        branch_mod::checkout(&r, name.trim())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn delete_branch(
    app: AppHandle,
    repo_path: String,
    name: String,
    force: bool,
) -> Result<(), String> {
    events::cmd(
        &app,
        &format!("git branch {}{name}", if force { "-D " } else { "-d " }),
    );
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        branch_mod::delete(&r, name.trim(), force)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn rename_branch(
    app: AppHandle,
    repo_path: String,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    events::cmd(&app, &format!("git branch -m {old_name} {new_name}"));
    tokio::task::spawn_blocking(move || {
        let r = repo::open(Path::new(&repo_path))?;
        branch_mod::rename(&r, old_name.trim(), new_name.trim())
    })
    .await
    .map_err(|e| e.to_string())?
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

#[tauri::command]
pub async fn init_repo(app: AppHandle, path: String) -> Result<repo::RepoInfo, String> {
    events::cmd(&app, &format!("git init -C {}", shell_quote(&path)));
    tokio::task::spawn_blocking(move || {
        let r = repo::init(std::path::Path::new(&path))?;
        repo::info(&r)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_gitignore(repo_path: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || ignore_mod::get(&repo_path))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn set_gitignore(repo_path: String, content: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || ignore_mod::set(&repo_path, &content))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn merge_branch(
    app: AppHandle,
    repo_path: String,
    branch_name: String,
) -> Result<merge::MergeResult, String> {
    events::cmd(&app, &format!("git merge {branch_name}"));
    tokio::task::spawn_blocking(move || merge::merge(&repo_path, &branch_name))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn list_mergeable_branches(repo_path: String) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || merge::list_mergeable_branches(&repo_path))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn reset_soft(
    app: AppHandle,
    repo_path: String,
    target: String,
) -> Result<(), String> {
    events::cmd(&app, &format!("git reset --soft {target}"));
    tokio::task::spawn_blocking(move || reset::reset_soft(&repo_path, &target))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn reset_mixed(
    app: AppHandle,
    repo_path: String,
    target: String,
) -> Result<(), String> {
    events::cmd(&app, &format!("git reset --mixed {target}"));
    tokio::task::spawn_blocking(move || reset::reset_mixed(&repo_path, &target))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn reset_hard(
    app: AppHandle,
    repo_path: String,
    target: String,
) -> Result<(), String> {
    events::cmd(&app, &format!("git reset --hard {target}"));
    tokio::task::spawn_blocking(move || reset::reset_hard(&repo_path, &target))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn revert_commit(
    app: AppHandle,
    repo_path: String,
    commit_oid: String,
) -> Result<(), String> {
    events::cmd(&app, &format!("git revert {commit_oid}"));
    tokio::task::spawn_blocking(move || reset::revert_commit(&repo_path, &commit_oid))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_log(
    repo_path: String,
    max_count: usize,
) -> Result<Vec<log_mod::LogEntry>, String> {
    tokio::task::spawn_blocking(move || log_mod::log(&repo_path, max_count))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn list_ssh_keys() -> Result<Vec<ssh::SshKey>, String> {
    tokio::task::spawn_blocking(|| ssh::list_keys())
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn generate_ssh_key(
    name: String,
    key_type: String,
    comment: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || ssh::generate_key(&name, &key_type, &comment))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_undo_point(repo_path: String) -> Result<undo::UndoPoint, String> {
    tokio::task::spawn_blocking(move || undo::head_info(&repo_path))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn undo_head(
    app: AppHandle,
    repo_path: String,
    keep_changes: bool,
) -> Result<undo::UndoPoint, String> {
    events::cmd(
        &app,
        &if keep_changes {
            "git reset --soft HEAD~1".to_string()
        } else {
            "git reset --mixed HEAD~1".to_string()
        },
    );
    tokio::task::spawn_blocking(move || undo::undo_last_commit(&repo_path, keep_changes))
        .await
        .map_err(|e| e.to_string())?
}
