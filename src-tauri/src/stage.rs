use git2::{IndexAddOption, Repository};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StagedSummary {
    pub files: usize,
    pub insertions: usize,
    pub deletions: usize,
}

pub fn stage(repo: &Repository, files: &[String]) -> Result<(), String> {
    let mut index = repo.index().map_err(|e| e.message().to_string())?;
    index
        .add_all(files.iter().map(|s| s.as_str()), IndexAddOption::DEFAULT, None)
        .map_err(|e| e.message().to_string())?;
    index.write().map_err(|e| e.message().to_string())
}

pub fn unstage(repo: &Repository, files: &[String]) -> Result<(), String> {
    match repo.head() {
        Ok(head) => {
            let commit = head.peel_to_commit().map_err(|e| e.message().to_string())?;
            let obj = commit.as_object();
            repo.reset_default(Some(obj), files.iter().map(|s| s.as_str()))
                .map_err(|e| e.message().to_string())
        }
        Err(_) => {
            let mut index = repo.index().map_err(|e| e.message().to_string())?;
            index
                .remove_all(files.iter().map(|s| s.as_str()), None)
                .map_err(|e| e.message().to_string())?;
            index.write().map_err(|e| e.message().to_string())
        }
    }
}

pub fn summary(repo: &Repository, files: &[String]) -> Result<StagedSummary, String> {
    if files.is_empty() {
        return Ok(StagedSummary {
            files: 0,
            insertions: 0,
            deletions: 0,
        });
    }

    let head_tree = repo.head().ok().and_then(|h| h.peel_to_tree().ok());

    let mut index = repo.index().map_err(err)?;
    index
        .add_all(files.iter().map(|s| s.as_str()), IndexAddOption::DEFAULT, None)
        .map_err(err)?;
    let tree_id = index.write_tree().map_err(err)?;
    let tree = repo.find_tree(tree_id).map_err(err)?;

    let mut opts = git2::DiffOptions::new();
    let diff = repo
        .diff_tree_to_tree(head_tree.as_ref(), Some(&tree), Some(&mut opts))
        .map_err(err)?;
    let stats = diff.stats().map_err(err)?;

    Ok(StagedSummary {
        files: stats.files_changed(),
        insertions: stats.insertions(),
        deletions: stats.deletions(),
    })
}

fn err(e: git2::Error) -> String {
    e.message().to_string()
}
