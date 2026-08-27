mod commands;
pub mod branch;
pub mod commit;
pub mod config;
pub mod credentials;
pub mod events;
pub mod ignore;
pub mod log;
pub mod merge;
pub mod pull;
pub mod push;
pub mod repo;
pub mod reset;
pub mod ssh;
pub mod stage;
pub mod undo;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
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
            commands::open_external,
            commands::save_credential,
            commands::load_credential,
            commands::delete_credential,
            commands::list_branches,
            commands::create_branch,
            commands::checkout_branch,
            commands::delete_branch,
            commands::rename_branch,
            commands::init_repo,
            commands::get_gitignore,
            commands::set_gitignore,
            commands::merge_branch,
            commands::list_mergeable_branches,
            commands::reset_soft,
            commands::reset_mixed,
            commands::reset_hard,
            commands::revert_commit,
            commands::get_log,
            commands::list_ssh_keys,
            commands::generate_ssh_key,
            commands::undo_head,
            commands::get_undo_point
        ])
        .run(tauri::generate_context!())
        .expect("failed to run gitwizard");
}
