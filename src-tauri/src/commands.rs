use crate::api::fetch_user_repositories;
use crate::models::Repository;

#[tauri::command]
pub async fn get_repos(username: String) -> Result<Vec<Repository>, String> {
    fetch_user_repositories(&username)
        .await
        .map_err(|e| e.to_string())
}
