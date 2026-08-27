use git2::{Branch, Repository, StatusOptions, Status};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoInfo {
    pub path: String,
    pub name: String,
    pub branch: Option<String>,
    pub ahead: usize,
    pub behind: usize,
    pub dirty_count: usize,
    pub remote_url: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FileKind {
    Added,
    Modified,
    Deleted,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileEntry {
    pub path: String,
    pub worktree: Option<FileKind>,
    pub staged: Option<FileKind>,
    pub conflicted: bool,
}

pub fn open(path: &Path) -> Result<Repository, String> {
    Repository::discover(path).map_err(|_| {
        format!(
            "'{}' 不是有效的 Git 仓库",
            path.to_string_lossy()
        )
    })
}

pub fn init(path: &Path) -> Result<Repository, String> {
    if path.join(".git").exists() {
        return Err("REPO_EXISTS: 该目录已是一个 Git 仓库".into());
    }
    Repository::init(path).map_err(|e| format!("INIT_FAIL: {}", e.message()))
}

pub fn info(repo: &Repository) -> Result<RepoInfo, String> {
    let workdir = repo.workdir().ok_or("不支持 bare 仓库")?;
    let path = workdir.to_string_lossy().into_owned();
    let name = workdir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.clone());

    let mut branch: Option<String> = None;
    let mut ahead = 0usize;
    let mut behind = 0usize;

    if let Ok(head) = repo.head() {
        branch = head.shorthand().map(|s| s.to_string());
        let local = head.target();
        if let Ok(upstream) = Branch::wrap(head).upstream() {
            if let (Some(l), Some(u)) = (local, upstream.get().target()) {
                if let Ok((a, b)) = repo.graph_ahead_behind(l, u) {
                    ahead = a;
                    behind = b;
                }
            }
        }
    }

    let remote_url = repo
        .find_remote("origin")
        .ok()
        .and_then(|r| r.url().map(|u| u.to_string()));

    let dirty_count = change_count(repo)?;

    Ok(RepoInfo {
        path,
        name,
        branch,
        ahead,
        behind,
        dirty_count,
        remote_url,
    })
}

pub fn status_entries(repo: &Repository) -> Result<Vec<FileEntry>, String> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(true)
        .exclude_submodules(true)
        .renames_head_to_index(true);

    let statuses = repo
        .statuses(Some(&mut opts))
        .map_err(|e| e.message().to_string())?;

    let mut out: Vec<FileEntry> = Vec::new();
    for entry in statuses.iter() {
        let s: Status = entry.status();
        let Some(raw) = entry.path() else { continue };
        let path = raw.replace('\\', "/");

        if s.contains(Status::CONFLICTED) {
            out.push(FileEntry {
                path,
                worktree: Some(FileKind::Modified),
                staged: None,
                conflicted: true,
            });
            continue;
        }

        let staged = index_kind(s);
        let worktree = wt_kind(s);
        if staged.is_none() && worktree.is_none() {
            continue;
        }
        out.push(FileEntry {
            path,
            worktree,
            staged,
            conflicted: false,
        });
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

fn index_kind(s: Status) -> Option<FileKind> {
    if s.contains(Status::INDEX_NEW) {
        Some(FileKind::Added)
    } else if s.contains(Status::INDEX_DELETED) {
        Some(FileKind::Deleted)
    } else if s.intersects(Status::INDEX_MODIFIED | Status::INDEX_RENAMED | Status::INDEX_TYPECHANGE)
    {
        Some(FileKind::Modified)
    } else {
        None
    }
}

fn wt_kind(s: Status) -> Option<FileKind> {
    if s.contains(Status::WT_NEW) {
        Some(FileKind::Added)
    } else if s.contains(Status::WT_DELETED) {
        Some(FileKind::Deleted)
    } else if s.intersects(Status::WT_MODIFIED | Status::WT_RENAMED | Status::WT_TYPECHANGE) {
        Some(FileKind::Modified)
    } else {
        None
    }
}

fn change_count(repo: &Repository) -> Result<usize, String> {
    let entries = status_entries(repo)?;
    Ok(entries.len())
}
