use serde::{Deserialize, Serialize};

const APP_NAME: &str = "gitwizard";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AppConfig {
    pub locale: String,
    pub user_name: Option<String>,
    pub user_email: Option<String>,
    pub auto_push: bool,
    pub recent_repos: Vec<String>,
    pub commit_prefixes: Vec<String>,
    pub update_proxy: Option<String>,
    pub auto_check_update: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            locale: "zh-CN".into(),
            user_name: None,
            user_email: None,
            auto_push: true,
            recent_repos: Vec::new(),
            commit_prefixes: ["feat", "fix", "docs", "chore", "refactor", "test"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            update_proxy: None,
            auto_check_update: true,
        }
    }
}

pub fn load() -> AppConfig {
    confy::load(APP_NAME, None).unwrap_or_default()
}

pub fn save(cfg: &AppConfig) -> Result<(), String> {
    confy::store(APP_NAME, None, cfg).map_err(|e| e.to_string())
}
