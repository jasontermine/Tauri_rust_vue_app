use crate::api::fetch_user_repositories;
use crate::models::Repository as RepoModel;
use git2::Repository as GitRepository;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::api::dialog::FileDialogBuilder;
use tauri::State;

pub struct DirectoryPath(pub Mutex<Option<String>>);

#[tauri::command]
pub async fn get_repos(username: String) -> Result<Vec<RepoModel>, String> {
    fetch_user_repositories(&username)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_directory() -> Result<Option<String>, String> {
    let (sender, receiver) = tokio::sync::oneshot::channel();

    FileDialogBuilder::new()
        .set_directory("/home")
        .pick_folder(move |selected_path| {
            let _ = sender.send(selected_path.map(|p| p.display().to_string()));
        });

    match receiver.await {
        Ok(Some(path)) => Ok(Some(path)),
        Ok(None) => Err("No directory selected".into()),
        Err(_) => Err("Failed to receive selected directory".into()),
    }
}

#[tauri::command]
pub async fn set_directory_path(path: String, state: State<'_, DirectoryPath>) -> Result<(), String> {
    let mut dir_path = state.0.lock().map_err(|e| e.to_string())?;
    *dir_path = Some(path);
    Ok(())
}

#[tauri::command]
pub async fn get_directory_path(state: State<'_, DirectoryPath>) -> Result<Option<String>, String> {
    let dir_path = state.0.lock().map_err(|e| e.to_string())?;
    Ok(dir_path.clone())
}

#[tauri::command]
pub async fn clone_repo(repo_url: String, state: State<'_, DirectoryPath>) -> Result<(), String> {
    let dir_path = state.0.lock().map_err(|e| e.to_string())?;
    let path = if let Some(ref path) = *dir_path {
        PathBuf::from(path)
    } else {
        return Err("No directory set".into());
    };

    match GitRepository::clone(&repo_url, &path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to clone repo: {}", e)),
    }
}
