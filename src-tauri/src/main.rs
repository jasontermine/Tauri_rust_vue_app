// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    name: String,
    html_url: String,
    description: Option<String>,
}

async fn fetch_user_repositories(username: &str) -> Result<Vec<Repository>, Error> {
    let url: String = format!("https://api.github.com/users/{}/repos", username);
    let client: Client = reqwest::Client::new();
    let response: Response = client
        .get(&url)
        .header("User-Agent", "request")
        .send()
        .await?;

    let repos: Vec<Repository> = response.json::<Vec<Repository>>().await?;
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
