use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    name: String,
    html_url: String,
    description: Option<String>,
}
