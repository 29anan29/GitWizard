use std::fs;
use std::path::Path;

pub fn get(repo_path: &str) -> Result<String, String> {
    let p = Path::new(repo_path).join(".gitignore");
    if p.exists() {
        fs::read_to_string(&p).map_err(|e| e.to_string())
    } else {
        Ok(String::new())
    }
}

pub fn set(repo_path: &str, content: &str) -> Result<(), String> {
    let p = Path::new(repo_path).join(".gitignore");
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&p, content).map_err(|e| e.to_string())
}
