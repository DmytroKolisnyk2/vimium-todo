use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::env;
use crate::auth::AuthenticationManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub is_completed: bool
}

#[derive(Clone)]
pub struct TaskManager {
    auth_manager: AuthenticationManager,
    client: Client,
    server_url: String,
}

impl TaskManager {
    pub fn new(auth_manager: AuthenticationManager) -> Self {
        Self {
            auth_manager,
            client: Client::new(),
            server_url: env::var("SERVER_URL").expect("SERVER_URL must be set"),
        }
    }

    pub async fn add_task(&self, name: &str, description: &str) -> Result<(), String> {
        if let Some(token) = self.auth_manager.get_token() {
            let response = self.client.post(format!("{}/api/tasks", self.server_url))
                .bearer_auth(token)
                .json(&serde_json::json!({
                    "data": {
                        "name": name,
                        "description": description,
                    }
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
        } else {
            Err("Not authenticated".to_string())
        }
    }

    pub async fn list_my_tasks(&self) -> Result<Vec<Task>, String> {
        if let Some(token) = self.auth_manager.get_token() {
            let response = self.client.get(format!("{}/api/tasks/my", self.server_url))
                .bearer_auth(token)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if response.status().is_success() {
                let tasks: Vec<Task> = response.json().await.map_err(|e| e.to_string())?;
                Ok(tasks)
            } else {
                let error_text = response.text().await.map_err(|e| e.to_string())?;
                Err(format!("Error: {}", error_text))
            }
        } else {
            Err("Not authenticated".to_string())
        }
    }

    pub async fn list_completed_tasks(&self) -> Result<Vec<Task>, String> {
        if let Some(token) = self.auth_manager.get_token() {
            let response = self.client.get(format!("{}/api/tasks/completed", self.server_url))
                .bearer_auth(token)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if response.status().is_success() {
                let tasks: Vec<Task> = response.json().await.map_err(|e| e.to_string())?;
                Ok(tasks)
            } else {
                let error_text = response.text().await.map_err(|e| e.to_string())?;
                Err(format!("Error: {}", error_text))
            }
        } else {
            Err("Not authenticated".to_string())
        }
    }

    pub async fn complete_task(&self, task_id: u32) -> Result<(), String> {
        if let Some(token) = self.auth_manager.get_token() {
            let response = self.client.put(format!("{}/api/tasks/{}", self.server_url, task_id))
                .bearer_auth(token)
                .json(&serde_json::json!({
                    "data": {
                        "isCompleted": true
                    }
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
        } else {
            Err("Not authenticated".to_string())
        }
    }

    pub async fn delete_task(&self, task_id: u32) -> Result<(), String> {
        if let Some(token) = self.auth_manager.get_token() {
            let response = self.client.delete(format!("{}/api/tasks/{}", self.server_url, task_id))
                .bearer_auth(token)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if response.status().is_success() {
                Ok(())
            } else {
                let error_text = response.text().await.map_err(|e| e.to_string())?;
                Err(format!("Error: {}", error_text))
            }
        } else {
            Err("Not authenticated".to_string())
        }
    }
}
