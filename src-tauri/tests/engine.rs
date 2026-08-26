use git2::Repository;
use gitwizard_lib::repo::FileKind;
use gitwizard_lib::{commit as cw, pull as pw, push as ph, repo as rw, stage as sw};
use std::fs;
use std::path::Path;
use std::sync::Arc;

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

fn noop_sideband() -> Arc<dyn Fn(&str) + Send + Sync> {
    Arc::new(|_: &str| {})
}

fn identity_of(repo_path: &Path) -> Option<(&'static str, &'static str)> {
    let _ = repo_path;
    Some(("Tester", "tester@example.com"))
}

struct PullSandbox {
    _guard: tempfile::TempDir,
    a: std::path::PathBuf,
    b: std::path::PathBuf,
}

fn setup_pull_sandbox() -> PullSandbox {
    let dir = tempfile::tempdir().unwrap();
    let bare = dir.path().join("server.git");
    Repository::init_bare(&bare).unwrap();

    let a = dir.path().join("a");
    let ra = Repository::clone(bare.to_str().unwrap(), &a).unwrap();
    {
        let mut cfg = ra.config().unwrap();
        cfg.set_str("user.name", "Tester").unwrap();
        cfg.set_str("user.email", "tester@example.com").unwrap();
        ra.set_head("refs/heads/master").unwrap();
    }
    drop(ra);

    write_file(&a, "f1.txt", "one\n");
    let ra = Repository::open(&a).unwrap();
    sw::stage(&ra, &["f1.txt".to_string()]).unwrap();
    cw::commit(&ra, "c1", None).unwrap();

    let url = bare.to_str().unwrap().to_string();
    ra.remote_set_url("origin", &url).unwrap();
    ph::push(
        &ra,
        "origin",
        "master",
        "master",
        None,
        None,
        Arc::new(|_, _| {}),
        noop_sideband(),
    )
    .unwrap();
    drop(ra);

    let b = dir.path().join("b");
    let rb = Repository::clone(bare.to_str().unwrap(), &b).unwrap();
    let mut cfg = rb.config().unwrap();
    cfg.set_str("user.name", "Tester-B").unwrap();
    cfg.set_str("user.email", "b@example.com").unwrap();
    drop(cfg);
    drop(rb);

    PullSandbox {
        _guard: dir,
        a,
        b,
    }
}

#[test]
fn pull_fast_forward_and_merge_and_dirty_guard() {
    let sbx = setup_pull_sandbox();

    let rb = Repository::open(&sbx.b).unwrap();
    let out = pw::pull(&rb, "origin", "master", None, None, identity_of(&sbx.b), noop_sideband())
        .unwrap_or_else(|e| panic!("first pull should be up_to_date, got: {e}"));
    assert_eq!(out.status, pw::PullStatus::UpToDate);

    write_file(&sbx.a, "f2.txt", "two\n");
    let ra = Repository::open(&sbx.a).unwrap();
    sw::stage(&ra, &["f2.txt".to_string()]).unwrap();
    cw::commit(&ra, "c2", None).unwrap();
    ph::push(
        &ra,
        "origin",
        "master",
        "master",
        None,
        None,
        Arc::new(|_, _| {}),
        noop_sideband(),
    )
    .unwrap();

    let out = pw::pull(&rb, "origin", "master", None, None, identity_of(&sbx.a), noop_sideband())
        .unwrap_or_else(|e| panic!("ff pull failed: {e}"));
    assert_eq!(out.status, pw::PullStatus::FastForward);
    assert!(sbx.b.join("f2.txt").exists());
    let entries = rw::status_entries(&rb).unwrap();
    assert!(entries.is_empty());

    write_file(&sbx.b, "b-only.txt", "from b\n");
    sw::stage(&rb, &["b-only.txt".to_string()]).unwrap();
    cw::commit(&rb, "b1", None).unwrap();

    write_file(&sbx.a, "a-only.txt", "from a\n");
    sw::stage(&ra, &["a-only.txt".to_string()]).unwrap();
    cw::commit(&ra, "a1", None).unwrap();
    ph::push(
        &ra,
        "origin",
        "master",
        "master",
        None,
        None,
        Arc::new(|_, _| {}),
        noop_sideband(),
    )
    .unwrap();

    let out = pw::pull(&rb, "origin", "master", None, None, identity_of(&sbx.b), noop_sideband())
        .unwrap_or_else(|e| panic!("merge pull failed: {e}"));
    assert_eq!(out.status, pw::PullStatus::Merged);

    let head = rb.head().and_then(|h| h.peel_to_commit()).unwrap();
    assert_eq!(head.parent_count(), 2);
    assert!(sbx.b.join("a-only.txt").exists());
    assert!(rw::status_entries(&rb).unwrap().is_empty());
}

#[test]
fn pull_refuses_dirty_worktree() {
    let sbx = setup_pull_sandbox();
    write_file(&sbx.b, "pending.txt", "dirty\n");
    let rb = Repository::open(&sbx.b).unwrap();
    let res = pw::pull(&rb, "origin", "master", None, None, None, noop_sideband());
    let msg = res.unwrap_err();
    assert!(msg.contains("WORKTREE_DIRTY"), "unexpected error: {msg}");
}

#[test]
fn branch_create_switch_rename_delete_flow() {
    use gitwizard_lib::branch as bw;

    let (_guard, root) = setup();
    write_file(&root, "init.txt", "base\n");
    let repo = Repository::open(&root).unwrap();
    sw::stage(&repo, &["init.txt".to_string()]).unwrap();
    cw::commit(&repo, "c0", None).unwrap();

    bw::create(&repo, "dev", false).unwrap_or_else(|e| panic!("create: {e}"));
    assert_eq!(bw::current_branch(&repo).unwrap(), "master");

    bw::checkout(&repo, "dev").unwrap_or_else(|e| panic!("checkout dev: {e}"));
    assert_eq!(bw::current_branch(&repo).unwrap(), "dev");

    write_file(&root, "dev.txt", "on dev\n");
    sw::stage(&repo, &["dev.txt".to_string()]).unwrap();
    cw::commit(&repo, "dev1", None).unwrap();

    write_file(&root, "pending.txt", "dirty\n");
    let err = bw::checkout(&repo, "master").unwrap_err();
    assert!(err.contains("WORKTREE_DIRTY"), "unexpected: {err}");
    std::fs::remove_file(root.join("pending.txt")).unwrap();

    bw::checkout(&repo, "master").unwrap_or_else(|e| panic!("back to master: {e}"));
    assert!(!root.join("dev.txt").exists());

    let err = bw::delete(&repo, "dev", false).unwrap_err();
    assert!(err.contains("BRANCH_UNMERGED"), "unexpected: {err}");

    bw::delete(&repo, "dev", true).unwrap_or_else(|e| panic!("force delete: {e}"));

    bw::create(&repo, "tmp-a", false).unwrap();
    bw::rename(&repo, "tmp-a", "tmp-b").unwrap_or_else(|e| panic!("rename: {e}"));
    assert!(bw::rename(&repo, "tmp-b", "master").is_err());
    bw::delete(&repo, "tmp-b", true).unwrap();

    let err = bw::delete(&repo, "master", false).unwrap_err();
    assert!(err.contains("BRANCH_CURRENT"), "unexpected: {err}");

    assert!(bw::validate_name("bad name").is_err());
    assert!(bw::validate_name("-lead").is_err());
    assert!(bw::validate_name("ok/feat-1").is_ok());
}
