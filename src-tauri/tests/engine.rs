use git2::Repository;
use gitwizard_lib::repo::FileKind;
use gitwizard_lib::{commit as cw, repo as rw, stage as sw};
use std::fs;
use std::path::Path;

fn setup() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("demo");
    fs::create_dir_all(&root).unwrap();
    let r = Repository::init(&root).unwrap();
    let mut cfg = r.config().unwrap();
    cfg.set_str("user.name", "Tester").unwrap();
    cfg.set_str("user.email", "tester@example.com").unwrap();
    drop(r);
    (dir, root)
}

fn write_file(root: &Path, rel: &str, content: &str) {
    let p = root.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, content).unwrap();
}

#[test]
fn status_stage_commit_unstage_flow() {
    let (_guard, root) = setup();
    write_file(&root, "src/main.rs", "fn main() {}\n");

    let repo = rw::open(&root).unwrap();
    let entries = rw::status_entries(&repo).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].path, "src/main.rs");
    assert_eq!(entries[0].worktree, Some(FileKind::Added));
    assert_eq!(entries[0].staged, None);

    sw::stage(&repo, &["src/main.rs".to_string()]).unwrap();

    let sum = sw::summary(&repo, &["src/main.rs".to_string()]).unwrap();
    assert_eq!(sum.files, 1);
    assert!(sum.insertions >= 1);

    let entries = rw::status_entries(&repo).unwrap();
    assert_eq!(entries[0].staged, Some(FileKind::Added));

    let oid = cw::commit(&repo, "feat: init project", None).unwrap();
    assert_eq!(oid.len(), 40);

    let info = rw::info(&repo).unwrap();
    assert_eq!(info.dirty_count, 0);
    assert!(info.branch.is_some());

    write_file(&root, "src/main.rs", "fn main() { println!(\"hi\"); }\n");
    let entries = rw::status_entries(&repo).unwrap();
    assert_eq!(entries[0].worktree, Some(FileKind::Modified));

    sw::stage(&repo, &["src/main.rs".to_string()]).unwrap();
    let entries = rw::status_entries(&repo).unwrap();
    assert_eq!(entries[0].staged, Some(FileKind::Modified));

    sw::unstage(&repo, &["src/main.rs".to_string()]).unwrap();
    let entries = rw::status_entries(&repo).unwrap();
    assert_eq!(entries[0].staged, None);
    assert_eq!(entries[0].worktree, Some(FileKind::Modified));
}

#[test]
fn unstage_on_unborn_head_removes_from_index() {
    let (_guard, root) = setup();
    write_file(&root, "README.md", "# demo\n");
    let repo = rw::open(&root).unwrap();

    sw::stage(&repo, &["README.md".to_string()]).unwrap();
    let entries = rw::status_entries(&repo).unwrap();
    assert_eq!(entries[0].staged, Some(FileKind::Added));

    sw::unstage(&repo, &["README.md".to_string()]).unwrap();
    let entries = rw::status_entries(&repo).unwrap();
    assert_eq!(entries[0].worktree, Some(FileKind::Added));
    assert_eq!(entries[0].staged, None);
}

#[test]
fn summary_on_selected_files_only() {
    let (_guard, root) = setup();
    write_file(&root, "a.txt", "line\nline\nline\n");
    write_file(&root, "b.txt", "other\n");
    let repo = rw::open(&root).unwrap();

    let sum = sw::summary(&repo, &["a.txt".to_string()]).unwrap();
    assert_eq!(sum.files, 1);
    assert_eq!(sum.insertions, 3);
}

#[test]
fn open_rejects_non_repo() {
    let dir = tempfile::tempdir().unwrap();
    assert!(rw::open(dir.path()).is_err());
}
