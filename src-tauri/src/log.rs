use crate::repo;
use git2::Sort;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub oid: String,
    pub short_oid: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub time: i64,
    pub is_merge: bool,
}

pub fn log(repo_path: &str, max_count: usize) -> Result<Vec<LogEntry>, String> {
    let repo = repo::open(Path::new(repo_path))?;
    let head = repo.head().map_err(err_msg)?;
    let head_oid = head.target().ok_or("HEAD is detached")?;

    let mut walk = repo.revwalk().map_err(err_msg)?;
    walk.push(head_oid).map_err(err_msg)?;
    walk.set_sorting(Sort::TIME).map_err(err_msg)?;
    walk.simplify_first_parent().map_err(err_msg)?;

    let mut entries = Vec::new();
    for (i, oid) in walk.enumerate() {
        if i >= max_count {
            break;
        }
        let oid = oid.map_err(err_msg)?;
        let commit = repo.find_commit(oid).map_err(err_msg)?;
        let author = commit.author();

        let oid_str = oid.to_string();
        entries.push(LogEntry {
            short_oid: oid_str[..7.min(oid_str.len())].to_string(),
            oid: oid_str,
            message: commit.summary().unwrap_or("").to_string(),
            author: author.name().unwrap_or("").to_string(),
            email: author.email().unwrap_or("").to_string(),
            time: author.when().seconds(),
            is_merge: commit.parent_count() > 1,
        });
    }

    Ok(entries)
}

fn err_msg(e: git2::Error) -> String {
    e.message().to_string()
}
