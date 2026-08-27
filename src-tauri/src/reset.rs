use crate::repo;
use git2::{MergeOptions, Repository, ResetType};
use std::path::Path;

pub fn reset_soft(repo_path: &str, target: &str) -> Result<(), String> {
    let repo = repo::open(Path::new(repo_path))?;
    let obj = revparse_single(&repo, target)?;
    repo.reset(&obj, ResetType::Soft, None).map_err(err_msg)
}

pub fn reset_mixed(repo_path: &str, target: &str) -> Result<(), String> {
    let repo = repo::open(Path::new(repo_path))?;
    let obj = revparse_single(&repo, target)?;
    repo.reset(&obj, ResetType::Mixed, None).map_err(err_msg)
}

pub fn reset_hard(repo_path: &str, target: &str) -> Result<(), String> {
    let repo = repo::open(Path::new(repo_path))?;
    let obj = revparse_single(&repo, target)?;
    repo.reset(&obj, ResetType::Hard, None).map_err(err_msg)
}

pub fn revert_commit(repo_path: &str, commit_oid: &str) -> Result<(), String> {
    let repo = repo::open(Path::new(repo_path))?;
    let oid = git2::Oid::from_str(commit_oid).map_err(err_msg)?;
    let commit = repo.find_commit(oid).map_err(err_msg)?;
    let head = repo.head().map_err(err_msg)?;
    let head_oid = head.target().ok_or("HEAD is detached")?;
    let parent = repo.find_commit(head_oid).map_err(err_msg)?;

    let mut merge_opts = MergeOptions::new();
    repo.revert_commit(&commit, &parent, 1, Some(&mut merge_opts))
        .map_err(err_msg)?;

    Ok(())
}

fn revparse_single<'a>(repo: &'a Repository, spec: &str) -> Result<git2::Object<'a>, String> {
    let obj = repo.revparse_single(spec).map_err(err_msg)?;
    repo.find_object(obj.id(), None).map_err(err_msg)
}

fn err_msg(e: git2::Error) -> String {
    e.message().to_string()
}
