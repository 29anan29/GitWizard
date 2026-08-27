use crate::repo;
use git2::ResetType;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoPoint {
    pub ref_name: String,
    pub oid: String,
    pub short_oid: String,
    pub message: String,
}

pub fn head_info(repo_path: &str) -> Result<UndoPoint, String> {
    let repo = repo::open(Path::new(repo_path))?;
    let head = repo.head().map_err(err_msg)?;
    let oid = head.target().ok_or("HEAD is detached")?;
    let commit = repo.find_commit(oid).map_err(err_msg)?;

    let oid_str = oid.to_string();
    Ok(UndoPoint {
        ref_name: head.shorthand().unwrap_or("HEAD").to_string(),
        oid: oid_str.clone(),
        short_oid: oid_str[..7.min(oid_str.len())].to_string(),
        message: commit.summary().unwrap_or("").to_string(),
    })
}

pub fn undo_last_commit(repo_path: &str, keep_changes: bool) -> Result<UndoPoint, String> {
    let repo = repo::open(Path::new(repo_path))?;
    let head = repo.head().map_err(err_msg)?;
    let head_oid = head.target().ok_or("HEAD has no target — nothing to undo")?;
    let head_commit = repo.find_commit(head_oid).map_err(err_msg)?;

    if head_commit.parent_count() == 0 {
        return Err("INITIAL_COMMIT: cannot undo the first commit".into());
    }

    let parent = head_commit.parent(0).map_err(err_msg)?;
    let parent_oid = parent.id();

    if keep_changes {
        repo.reset(
            &repo
                .find_object(parent_oid, None)
                .map_err(err_msg)?,
            ResetType::Soft,
            None,
        )
        .map_err(err_msg)?;
    } else {
        repo.reset(
            &repo
                .find_object(parent_oid, None)
                .map_err(err_msg)?,
            ResetType::Mixed,
            None,
        )
        .map_err(err_msg)?;
    }

    let new_head = repo.head().map_err(err_msg)?;
    let new_oid = new_head
        .target()
        .ok_or("new HEAD has no target")?;
    let new_commit = repo.find_commit(new_oid).map_err(err_msg)?;

    let oid_str = new_oid.to_string();
    Ok(UndoPoint {
        ref_name: new_head.shorthand().unwrap_or("HEAD").to_string(),
        oid: oid_str.clone(),
        short_oid: oid_str[..7.min(oid_str.len())].to_string(),
        message: new_commit.summary().unwrap_or("").to_string(),
    })
}

fn err_msg(e: git2::Error) -> String {
    e.message().to_string()
}
