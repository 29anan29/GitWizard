use crate::repo;
use git2::{BranchType, MergeOptions};
use git2::build::CheckoutBuilder;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeResult {
    pub status: String,
    pub conflicts: Vec<String>,
}

pub fn merge(repo_path: &str, branch_name: &str) -> Result<MergeResult, String> {
    let repo = repo::open(Path::new(repo_path))?;
    let head = repo.head().map_err(err_msg)?;
    let head_oid = head.target().ok_or("HEAD is detached")?;

    let branch_ref = repo
        .find_branch(branch_name, BranchType::Local)
        .map_err(|_| format!("BRANCH_NOT_FOUND:{branch_name}"))?;

    let branch_oid = branch_ref
        .get()
        .target()
        .ok_or("branch has no target")?;

    if head_oid == branch_oid {
        return Ok(MergeResult {
            status: "already_up_to_date".into(),
            conflicts: vec![],
        });
    }

    if repo
        .graph_descendant_of(head_oid, branch_oid)
        .unwrap_or(false)
    {
        return Ok(MergeResult {
            status: "already_up_to_date".into(),
            conflicts: vec![],
        });
    }

    if repo
        .graph_descendant_of(branch_oid, head_oid)
        .unwrap_or(false)
    {
        let obj = branch_ref
            .get()
            .peel(git2::ObjectType::Tree)
            .map_err(err_msg)?;
        repo.checkout_tree(&obj, None)
            .map_err(err_msg)?;
        repo.reference("HEAD", branch_oid, true, "fast-forward")
            .map_err(err_msg)?;
        return Ok(MergeResult {
            status: "fast_forward".into(),
            conflicts: vec![],
        });
    }

    let annotated = repo.find_annotated_commit(branch_oid).map_err(err_msg)?;

    let mut merge_opts = MergeOptions::new();
    let mut checkout = CheckoutBuilder::new();
    checkout.allow_conflicts(true);
    repo.merge(&[&annotated], Some(&mut merge_opts), Some(&mut checkout))
        .map_err(err_msg)?;

    let conflicted: Vec<String> = crate::repo::status_entries(&repo)?
        .into_iter()
        .filter(|e| e.conflicted)
        .map(|e| e.path)
        .collect();

    if !conflicted.is_empty() {
        rollback_merge(&repo)?;
        return Ok(MergeResult {
            status: "conflict".into(),
            conflicts: conflicted,
        });
    }

    let sig = repo.signature().map_err(err_msg)?;
    let mut index = repo.index().map_err(err_msg)?;
    index.write().map_err(err_msg)?;
    let tree_id = index.write_tree().map_err(err_msg)?;
    let tree = repo.find_tree(tree_id).map_err(err_msg)?;
    let head_commit = repo.find_commit(head_oid).map_err(err_msg)?;
    let branch_commit = repo.find_commit(branch_oid).map_err(err_msg)?;

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &format!("Merge branch '{branch_name}'"),
        &tree,
        &[&head_commit, &branch_commit],
    )
    .map_err(err_msg)?;

    repo.cleanup_state().map_err(err_msg)?;

    Ok(MergeResult {
        status: "merged".into(),
        conflicts: vec![],
    })
}

fn rollback_merge(repo: &git2::Repository) -> Result<(), String> {
    let head_commit = repo
        .head()
        .and_then(|h| h.peel_to_commit())
        .map_err(err_msg)?;
    repo.reset(head_commit.as_object(), git2::ResetType::Hard, None)
        .map_err(err_msg)?;
    let _ = repo.cleanup_state();
    Ok(())
}

pub fn list_mergeable_branches(repo_path: &str) -> Result<Vec<String>, String> {
    let repo = repo::open(Path::new(repo_path))?;
    let head = repo.head().map_err(err_msg)?;
    let head_oid = head.target().ok_or("HEAD is detached")?;

    let branches = repo
        .branches(Some(BranchType::Local))
        .map_err(err_msg)?;

    let mut result = Vec::new();

    for branch_result in branches {
        let (branch, _) = branch_result.map_err(err_msg)?;
        if let Some(name) = branch.name().map_err(err_msg)? {
            if let Some(oid) = branch.get().target() {
                if oid != head_oid {
                    if !repo.graph_descendant_of(oid, head_oid).unwrap_or(false) {
                        result.push(name.to_string());
                    }
                }
            }
        }
    }

    Ok(result)
}

fn err_msg(e: git2::Error) -> String {
    e.message().to_string()
}
