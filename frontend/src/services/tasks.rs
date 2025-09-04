use chrono::DateTime;
use gloo::net::http::Request;
use serde::{Serialize, Deserialize};
use super::{API_URL, auth::get_token};

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskDto {
    pub title: String,
    pub category: String,
    pub description: String,
    pub begin_date: DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    pub task_type: String,
}

pub enum TaskResult {
    Success(TaskDto),
    InvalidFields,
    NetworkError(String),
}

pub async fn create_task(task_info: &TaskDto) -> TaskResult {
    let url = format!("{}/tasks", API_URL);
    let token = get_token();
    
    // Check if we have a valid token
    if token.token.is_empty() {
        return TaskResult::NetworkError("No authentication token found".into());
    }

    match Request::post(&url)
        .header("Authorization", &format!("Bearer {}", token.token))
        .header("Content-Type", "application/json")
        .json(&task_info)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.status() {
            200 | 201 => {
                let task: TaskDto = response.json().await.unwrap();
                TaskResult::Success(task)
            }
            400 => TaskResult::InvalidFields,
            401 => TaskResult::NetworkError("Unauthorized - please login again".into()),
            422 => {
                // Try to get the response body for more details
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown validation error".to_string());
                TaskResult::NetworkError(format!("Validation error (422): {}", error_text))
            },
            _ => {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                TaskResult::NetworkError(format!("HTTP {}: {}", response.status(), error_text))
            },
        },
        Err(e) => TaskResult::NetworkError(e.to_string()),
    }
}