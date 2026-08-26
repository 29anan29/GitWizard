use crate::repo::status_entries;
use git2::build::CheckoutBuilder;
use git2::{BranchType, Oid, Repository};

fn err(e: git2::Error) -> String {
    e.message().to_string()
}

pub fn validate_name(name: &str) -> Result<(), String> {
    let n = name.trim();
    if n.is_empty() || n.len() > 120 || n == "HEAD" || n.ends_with(".lock") {
        return Err("BRANCH_NAME: invalid branch name".into());
    }
    if n.starts_with('-') || n.starts_with('/') || n.ends_with('/') {
        return Err("BRANCH_NAME: invalid branch name".into());
    }
    for bad in ["..", "//", "@{"] {
        if n.contains(bad) {
            return Err("BRANCH_NAME: invalid branch name".into());
        }
    }
    for ch in n.chars() {
        if matches!(
            ch,
            ' ' | '\t'
                | '~'
                | '^'
                | ':'
                | '?'
                | '*'
                | '['
                | '\\'
                | '\u{7f}'
        ) || ch.is_control()
        {
            return Err("BRANCH_NAME: invalid branch name".into());
        }
    }
    Ok(())
}

pub fn current_branch(repo: &Repository) -> Result<String, String> {
    let head = repo.head().map_err(|_| "DETACHED_HEAD".to_string())?;
    head.shorthand()
        .map(|s| s.to_string())
        .ok_or_else(|| "DETACHED_HEAD".to_string())
}

fn ensure_clean(repo: &Repository) -> Result<(), String> {
    let entries = status_entries(repo)?;
    if entries.iter().any(|e| e.worktree.is_some() || e.staged.is_some()) {
        return Err("WORKTREE_DIRTY: 工作区有未提交的修改".into());
    }
    Ok(())
}

fn branch_exists(repo: &Repository, name: &str) -> bool {
    repo.find_branch(name, BranchType::Local).is_ok()
}

fn do_checkout(repo: &Repository, name: &str) -> Result<(), String> {
    repo.set_head(&format!("refs/heads/{name}"))
        .map_err(err)?;
    repo.checkout_head(Some(CheckoutBuilder::new().force()))
        .map_err(err)?;
    Ok(())
}

pub fn create(repo: &Repository, name: &str, switch: bool) -> Result<(), String> {
    validate_name(name)?;
    if branch_exists(repo, name) {
        return Err(format!("BRANCH_EXISTS:{name}"));
    }
    let commit = repo
        .head()
        .and_then(|h| h.peel_to_commit())
        .map_err(err)?;
    repo.branch(name, &commit, false).map_err(err)?;
    if switch {
        ensure_clean(repo)?;
        do_checkout(repo, name)?;
    }
    Ok(())
}

pub fn checkout(repo: &Repository, name: &str) -> Result<(), String> {
    validate_name(name)?;
    ensure_clean(repo)?;

    if !branch_exists(repo, name) {
        let remote_ref_name = format!("refs/remotes/origin/{name}");
        let remote_ref = repo
            .find_reference(&remote_ref_name)
            .map_err(|_| format!("BRANCH_NOT_FOUND:{name}"))?;
        let commit_oid: Oid = remote_ref.target().ok_or_else(|| {
            format!("BRANCH_NOT_FOUND:{name}")
        })?;
        let commit = repo.find_commit(commit_oid).map_err(err)?;
        repo.branch(name, &commit, false).map_err(err)?;
        if let Ok(mut local) = repo.find_branch(name, BranchType::Local) {
            let upstream = format!("origin/{name}");
            let _ = local.set_upstream(Some(&upstream));
        }
    }

    do_checkout(repo, name)
}

pub fn delete(repo: &Repository, name: &str, force: bool) -> Result<(), String> {
    validate_name(name)?;
    if current_branch(repo)? == name {
        return Err("BRANCH_CURRENT: cannot delete the checked out branch".into());
    }

    let mut rf = repo
        .find_reference(&format!("refs/heads/{name}"))
        .map_err(|_| format!("BRANCH_NOT_FOUND:{name}"))?;

    if !force && !is_merged_elsewhere(repo, name)? {
        return Err("BRANCH_UNMERGED: branch has unmerged commits".into());
    }

    rf.delete().map_err(err)?;
    Ok(())
}

fn is_ancestor(repo: &Repository, tip: Oid, head: Oid) -> Result<bool, String> {
    let mut walk = repo.revwalk().map_err(err)?;
    walk.push(head).map_err(err)?;
    for oid in walk {
        if oid.map_err(err)? == tip {
            return Ok(true);
        }
    }
    Ok(false)
}

fn is_merged_elsewhere(repo: &Repository, name: &str) -> Result<bool, String> {
    let tip_ref = repo
        .find_reference(&format!("refs/heads/{name}"))
        .map_err(err)?;
    let Some(tip) = tip_ref.target() else {
        return Ok(false);
    };

    let heads = |bt: BranchType| -> Result<Vec<Oid>, String> {
        Ok(repo
            .branches(Some(bt))
            .map_err(err)?
            .filter_map(|b| b.ok())
            .filter(|(b, _)| b.name().ok().flatten() != Some(name))
            .filter_map(|(b, _)| b.get().target())
            .collect())
    };

    for head_oid in heads(BranchType::Local)? {
        if head_oid == tip || is_ancestor(repo, tip, head_oid)? {
            return Ok(true);
        }
    }
    for head_oid in heads(BranchType::Remote)? {
        if head_oid == tip || is_ancestor(repo, tip, head_oid)? {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn rename(repo: &Repository, old: &str, new: &str) -> Result<(), String> {
    validate_name(new)?;
    if old == new {
        return Ok(());
    }
    if branch_exists(repo, new) {
        return Err(format!("BRANCH_EXISTS:{new}"));
    }
    if !branch_exists(repo, old) {
        return Err(format!("BRANCH_NOT_FOUND:{old}"));
    }

    let mut old_ref = repo.find_reference(&format!("refs/heads/{old}")).map_err(err)?;
    let oid = old_ref.target().ok_or_else(|| err(git2::Error::from_str(
        "reference has no target",
    )))?;

    repo.reference(
        &format!("refs/heads/{new}"),
        oid,
        true,
        &format!("branch: renamed {old} to {new}"),
    )
    .map_err(err)?;
    old_ref.delete().map_err(err)?;

    if current_branch(repo)? == old {
        repo.set_head(&format!("refs/heads/{new}")).map_err(err)?;
    }
    Ok(())
}
