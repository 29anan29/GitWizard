use crate::push::auth_callbacks;
use crate::repo::status_entries;
use git2::{AnnotatedCommit, MergeOptions, Repository, ResetType, build::CheckoutBuilder};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PullStatus {
    UpToDate,
    FastForward,
    Merged,
    Conflict,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PullOutcome {
    pub status: PullStatus,
    pub conflicts: Vec<String>,
    pub fetched_oid: Option<String>,
}

type SidebandFn = Arc<dyn Fn(&str) + Send + Sync>;

pub fn pull(
    repo: &Repository,
    remote_name: &str,
    branch: &str,
    username: Option<String>,
    password: Option<String>,
    identity: Option<(&str, &str)>,
    sideband: SidebandFn,
) -> Result<PullOutcome, String> {
    let existing = status_entries(repo)?;
    if existing.iter().any(|e| e.worktree.is_some() || e.staged.is_some()) {
        return Err("WORKTREE_DIRTY: 工作区有未提交的修改".to_string());
    }

    let head = repo.head().map_err(|_| {
        "DETACHED_HEAD: 当前不在任何分支上".to_string()
    })?;
    let head_ref_name = head.name().ok_or("无法解析 HEAD 引用")?.to_string();

    let mut remote = repo
        .find_remote(remote_name)
        .map_err(|e| format!("找不到远端 '{}': {}", remote_name, e.message()))?;

    let mut cb = auth_callbacks(username, password);
    let sb = Arc::clone(&sideband);
    cb.sideband_progress(move |data| {
        let text = String::from_utf8_lossy(data);
        for line in text.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                sb(trimmed);
            }
        }
        true
    });

    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts.remote_callbacks(cb);

    let refspec = format!(
        "refs/heads/{b}:refs/remotes/{r}/{b}",
        b = branch,
        r = remote_name
    );
    remote
        .fetch(&[refspec.as_str()], Some(&mut fetch_opts), None)
        .map_err(classify_fetch_err)?;

    let fetch_head = repo.find_reference("FETCH_HEAD").map_err(err)?;
    let annotated: AnnotatedCommit = repo.reference_to_annotated_commit(&fetch_head).map_err(err)?;
    let fetched_oid = annotated.id().to_string();
    let fetched_commit = repo.find_commit(annotated.id()).map_err(err)?;

    let (analysis, _) = repo.merge_analysis(&[&annotated]).map_err(err)?;

    if analysis.is_up_to_date() {
        return Ok(PullOutcome {
            status: PullStatus::UpToDate,
            conflicts: vec![],
            fetched_oid: Some(fetched_oid),
        });
    }

    if analysis.is_fast_forward() {
        let tree = fetched_commit.tree().map_err(err)?;
        repo.checkout_tree(tree.as_object(), Some(CheckoutBuilder::new().force()))
            .map_err(err)?;
        let mut href = repo.find_reference(&head_ref_name).map_err(err)?;
        href.set_target(fetched_commit.id(), "pull: fast-forward")
            .map_err(err)?;
        repo.set_head(&head_ref_name).map_err(err)?;
        return Ok(PullOutcome {
            status: PullStatus::FastForward,
            conflicts: vec![],
            fetched_oid: Some(fetched_oid),
        });
    }

    let mut merge_opts = MergeOptions::new();
    let mut checkout = CheckoutBuilder::new();
    checkout.allow_conflicts(true);
    repo.merge(&[&annotated], Some(&mut merge_opts), Some(&mut checkout))
        .map_err(err)?;

    let conflicted: Vec<String> = status_entries(repo)?
        .into_iter()
        .filter(|e| e.conflicted)
        .map(|e| e.path)
        .collect();

    if !conflicted.is_empty() {
        rollback_merge(repo)?;
        return Ok(PullOutcome {
            status: PullStatus::Conflict,
            conflicts: conflicted,
            fetched_oid: Some(fetched_oid),
        });
    }

    let mut index = repo.index().map_err(err)?;
    index.write().map_err(err)?;
    let tree_id = index.write_tree().map_err(err)?;
    let tree = repo.find_tree(tree_id).map_err(err)?;

    let sig = crate::commit::signature(repo, identity)?;
    let ours = repo.head().and_then(|h| h.peel_to_commit()).map_err(err)?;

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "merge: sync remote changes",
        &tree,
        &[&ours, &fetched_commit],
    )
    .map_err(err)?;
    repo.cleanup_state().map_err(err)?;

    Ok(PullOutcome {
        status: PullStatus::Merged,
        conflicts: vec![],
        fetched_oid: Some(fetched_oid),
    })
}

fn rollback_merge(repo: &Repository) -> Result<(), String> {
    let head_commit = repo.head().and_then(|h| h.peel_to_commit()).map_err(err)?;
    repo.reset(head_commit.as_object(), ResetType::Hard, None)
        .map_err(err)?;
    let _ = repo.cleanup_state();
    Ok(())
}

fn classify_fetch_err(e: git2::Error) -> String {
    let msg = e.message().to_string();
    let low = msg.to_lowercase();
    if low.contains("authentication") || low.contains("credential") || low.contains("401") {
        format!("AUTH:{msg}")
    } else {
        format!("NET:{msg}")
    }
}

fn err(e: git2::Error) -> String {
    e.message().to_string()
}
