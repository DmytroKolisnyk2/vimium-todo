use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn register(username: &str, email: &str, password: &str) -> Result<(), String> {
        let client = Client::new();
        let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
        let response = client.post(format!("{}/api/auth/local/register", server_url))
            .json(&serde_json::json!({
                "username": username,
                "email": email,
                "password": password
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await.map_err(|e| e.to_string())?;
            Err(format!("Error: {}", error_text))
        }
    }
}
