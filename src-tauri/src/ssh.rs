use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SshKey {
    pub name: String,
    pub path: String,
    pub key_type: String,
    pub is_default: bool,
}

fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(PathBuf::from)
}

pub fn list_keys() -> Result<Vec<SshKey>, String> {
    let ssh_dir = home_dir()
        .ok_or("Cannot find home directory")?
        .join(".ssh");

    if !ssh_dir.exists() {
        return Ok(vec![]);
    }

    let mut keys = Vec::new();
    let entries = fs::read_dir(&ssh_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().into_owned();

        if name.starts_with('.')
            || name == "known_hosts"
            || name == "config"
            || name.ends_with(".pub")
        {
            continue;
        }

        let path = entry.path();
        let pub_path = PathBuf::from(format!("{}.pub", path.display()));

        let key_type = if pub_path.exists() {
            fs::read_to_string(&pub_path)
                .map(|c| {
                    c.split_whitespace()
                        .next()
                        .unwrap_or("unknown")
                        .to_string()
                })
                .unwrap_or_else(|_| "unknown".to_string())
        } else {
            "unknown".to_string()
        };

        keys.push(SshKey {
            name,
            path: path.display().to_string(),
            key_type,
            is_default: false,
        });
    }

    for key in &mut keys {
        if key.name == "id_rsa" || key.name == "id_ed25519" || key.name == "id_ecdsa" {
            key.is_default = true;
        }
    }

    Ok(keys)
}

pub fn generate_key(name: &str, key_type: &str, comment: &str) -> Result<String, String> {
    let ssh_dir = home_dir()
        .ok_or("Cannot find home directory")?
        .join(".ssh");

    fs::create_dir_all(&ssh_dir).map_err(|e| e.to_string())?;

    let key_path = ssh_dir.join(name);
    let key_type_arg = match key_type {
        "ed25519" => "-t ed25519",
        "ecdsa" => "-t ecdsa -b 521",
        _ => "-t rsa -b 4096",
    };

    let output = std::process::Command::new("ssh-keygen")
        .args(key_type_arg.split_whitespace())
        .arg("-f")
        .arg(&key_path)
        .arg("-C")
        .arg(comment)
        .arg("-N")
        .arg("")
        .output()
        .map_err(|e| format!("Failed to run ssh-keygen: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("ssh-keygen failed: {stderr}"));
    }

    Ok(key_path.display().to_string())
}
