use git2::{Cred, CredentialType, PushOptions, RemoteCallbacks, Repository};
use std::sync::Arc;

type ProgressFn = Arc<dyn Fn(usize, usize) + Send + Sync>;
type SidebandFn = Arc<dyn Fn(&str) + Send + Sync>;

pub(crate) fn auth_callbacks(
    username: Option<String>,
    password: Option<String>,
) -> RemoteCallbacks<'static> {
    let mut attempts: u32 = 0;
    let mut cb = RemoteCallbacks::new();
    cb.credentials(move |_url, _user_from_url, allowed| {
        attempts += 1;
        if attempts > 6 {
            return Err(git2::Error::from_str("认证失败：凭据被拒绝"));
        }
        if allowed.contains(CredentialType::USER_PASS_PLAINTEXT) {
            if attempts >= 3 {
                if let (Some(u), Some(p)) = (username.as_deref(), password.as_deref()) {
                    if !p.is_empty() {
                        return Cred::userpass_plaintext(u, p);
                    }
                }
                return Err(git2::Error::from_str("需要提供访问令牌或密码"));
            }
            return Cred::username(username.as_deref().unwrap_or("git"));
        }
        if allowed.contains(CredentialType::DEFAULT) {
            return Cred::default();
        }
        Err(git2::Error::from_str("不支持的凭据类型"))
    });
    cb
}

pub fn push(
    repo: &Repository,
    remote_name: &str,
    branch: &str,
    username: Option<String>,
    password: Option<String>,
    progress: ProgressFn,
    sideband: SidebandFn,
) -> Result<(), String> {
    let mut remote = repo
        .find_remote(remote_name)
        .map_err(|e| format!("找不到远端 '{}': {}", remote_name, e.message()))?;

    let mut cb = auth_callbacks(username, password);

    cb.push_transfer_progress(move |cur, total, _bytes| progress(cur, total));
    let sideband2 = Arc::clone(&sideband);
    cb.sideband_progress(move |data| {
        let text = String::from_utf8_lossy(data);
        for line in text.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                sideband2(trimmed);
            }
        }
        true
    });

    let mut opts = PushOptions::new();
    opts.remote_callbacks(cb);
    opts.packbuilder_parallelism(0);

    let refspec = format!("refs/heads/{b}:refs/heads/{b}", b = branch);
    remote
        .push(&[refspec.as_str()], Some(&mut opts))
        .map_err(classify_push_err)?;

    Ok(())
}

fn classify_push_err(e: git2::Error) -> String {
    let msg = e.message().to_string();
    let low = msg.to_lowercase();
    if low.contains("non-fast-forward")
        || low.contains("fetch first")
        || low.contains("rejected")
        || low.contains("behind")
    {
        format!("NON_FF:{msg}")
    } else {
        msg
    }
}
