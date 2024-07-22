use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub jwt: String,
}

#[derive(Clone)]
pub struct AuthenticationManager {
    token: Arc<Mutex<Option<String>>>,
    client: Client,
    server_url: String,
}

impl AuthenticationManager {
    pub fn new() -> Self {
        Self {
            token: Arc::new(Mutex::new(None)),
            client: Client::new(),
            server_url: env::var("SERVER_URL").expect("SERVER_URL must be set"),
        }
    }

    pub async fn login(&self, identifier: &str, password: &str) -> Result<String, String> {
        let response = self.client.post(format!("{}/api/auth/local", self.server_url))
            .json(&serde_json::json!({
                "identifier": identifier,
                "password": password
            }))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            let auth_response: AuthResponse = response.json().await.map_err(|e| e.to_string())?;
            let token = auth_response.jwt.clone();
            *self.token.lock().unwrap() = Some(token.clone());
            Ok(token)
        } else {
            let error_text = response.text().await.map_err(|e| e.to_string())?;
            Err(format!("Error: {}", error_text))
        }
    }

    pub fn logout(&self) {
        *self.token.lock().unwrap() = None;
    }

    pub fn get_token(&self) -> Option<String> {
        self.token.lock().unwrap().clone()
    }
}
