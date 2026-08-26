mod commands;
pub mod commit;
pub mod config;
pub mod events;
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
            commands::get_config,
            commands::save_config
        ])
        .run(tauri::generate_context!())
        .expect("failed to run gitwizard");
}
