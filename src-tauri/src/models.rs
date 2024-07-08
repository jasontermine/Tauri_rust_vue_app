use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
}

pub struct DirectoryPath(pub Mutex<Option<String>>);
