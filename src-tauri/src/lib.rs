mod commands;
pub mod commit;
pub mod config;
pub mod events;
pub mod pull;
pub mod push;
pub mod repo;
pub mod stage;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::open_repo_dialog,
            commands::validate_repo,
            commands::get_status,
            commands::stage_files,
            commands::unstage_files,
            commands::staged_summary,
            commands::commit_repo,
            commands::push_remote,
            commands::pull_branch,
            commands::get_config,
            commands::save_config,
            commands::check_updates,
            commands::open_external
        ])
        .run(tauri::generate_context!())
        .expect("failed to run gitwizard");
}
