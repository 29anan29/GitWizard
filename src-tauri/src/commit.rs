use git2::{Commit, Repository, Signature};

pub fn commit(
    repo: &Repository,
    message: &str,
    fallback_identity: Option<(&str, &str)>,
) -> Result<String, String> {
    let sig = signature(repo, fallback_identity)?;

    let mut index = repo.index().map_err(err)?;
    let tree_id = index.write_tree().map_err(err)?;
    let tree = repo.find_tree(tree_id).map_err(err)?;

    let parents: Vec<Commit> = match repo.head() {
        Ok(head) => vec![head.peel_to_commit().map_err(err)?],
        Err(_) => Vec::new(),
    };
    let parent_refs: Vec<&Commit> = parents.iter().collect();

    let oid = repo
        .commit(Some("HEAD"), &sig, &sig, message, &tree, &parent_refs)
        .map_err(err)?;

    Ok(oid.to_string())
}

fn signature<'r>(
    repo: &'r Repository,
    fallback_identity: Option<(&str, &str)>,
) -> Result<Signature<'r>, String> {
    if let Ok(s) = repo.signature() {
        return Ok(s);
    }
    if let Some((name, email)) = fallback_identity {
        return Signature::now(name, email).map_err(err);
    }
    Err("缺少提交者身份：请在设置中填写用户名和邮箱，或配置 git 全局 user.name / user.email".to_string())
}

fn err(e: git2::Error) -> String {
    e.message().to_string()
}
