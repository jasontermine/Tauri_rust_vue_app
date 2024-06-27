use crate::models::Repository;

use reqwest::{Client, Error, Response};

pub async fn fetch_user_repositories(username: &str) -> Result<Vec<Repository>, Error> {
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
