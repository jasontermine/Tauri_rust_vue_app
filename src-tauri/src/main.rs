use std::sync::Mutex;

mod commands;
mod models;
mod api; // Ensure you have this module where fetch_user_repositories is defined

use models::DirectoryPath;

fn main() {
    tauri::Builder::default()
        .manage(DirectoryPath(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            commands::get_repos,
            commands::open_directory,
            commands::set_directory_path,
            commands::get_directory_path,
            commands::clone_repo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
