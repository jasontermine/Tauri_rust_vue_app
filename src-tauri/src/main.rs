// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    name: String,
    html_url: String,
    description: Option<String>,
}

async fn fetch_user_repositories(username: &str) -> Result<Vec<Repository>, Error> {
    let url = format!("https://api.github.com/users/{}/repos", username);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "request")
        .send()
        .await?;

    let repos = response.json::<Vec<Repository>>().await?;
    Ok(repos)
}

#[tauri::command]
async fn get_repos(username: String) -> Result<Vec<Repository>, String> {
    fetch_user_repositories(&username)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_repos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
