use chrono::DateTime;
use gloo::net::http::Request;
use serde::{Serialize, Deserialize};
use crate::types::Task;

use super::{API_URL, auth::get_token};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskDto {
    pub title: String,
    pub category: String,
    pub description: String,
    pub begin_date: DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    pub task_type: String,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TaskUpdateDto {
    pub title: String,
    pub description: String,
    pub status: String,
}

pub enum TaskResult {
    Success(Task),
    InvalidFields,
    NetworkError(String),
}

pub async fn delete_task(_task_id: u32) -> Result<(), String> {
    let url = format!("{}/tasks/{}", API_URL, _task_id as i32);
    let token = get_token();
    if token.token.is_empty() {
        return Err("No authentication token found".to_string());
    }
    match Request::delete(&url)
        .header("Authorization", &format!("Bearer {}", token.token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 || response.status() == 204 {
                Ok(())
            } else {
                // Get the error message from the response body
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                let error_msg = format!("Failed to delete task: HTTP {} - {}", response.status(), error_text);
                web_sys::console::log_1(&error_msg.clone().into()); // Log the full error
                Err(error_msg)
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn get_all_tasks() -> Result<Vec<Task>, String> {
    let url = format!("{}/tasks", API_URL);
    let token = get_token();

    if token.token.is_empty() {
        return Err("No authentication token found".to_string());
    }

    match Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token.token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<Task>>().await {
                    Ok(tasks) => {
                        web_sys::console::log_1(&format!("Fetched {} tasks", tasks.len()).into());
                        Ok(tasks)
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to parse tasks JSON: {}", e);
                        web_sys::console::log_1(&error_msg.clone().into());
                        Err(error_msg)
                    }
                }
            } else {
                let error_msg = format!("Failed to fetch tasks: HTTP {}", response.status());
                web_sys::console::log_1(&error_msg.clone().into());
                Err(error_msg)
            }
        }
        Err(e) => {
            let error_msg = format!("Network error: {}", e);
            web_sys::console::log_1(&error_msg.clone().into());
            Err(error_msg)
        }
    }
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
                let task: Task = response.json().await.unwrap();
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

pub async fn update_task_with_dto(task_id: u32, task_dto: TaskUpdateDto) -> Result<(), String> {
    let url = format!("{}/tasks/{}", API_URL, task_id);
    let token = get_token();
    if token.token.is_empty() {
        return Err("No authentication token found".to_string());
    }

    match Request::put(&url)
        .header("Authorization", &format!("Bearer {}", token.token))
        .header("Content-Type", "application/json")
        .json(&task_dto)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 || response.status() == 204 {
                Ok(())
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                let error_msg = format!("Failed to update task with DTO: HTTP {} - {}", response.status(), error_text);
                web_sys::console::log_1(&error_msg.clone().into());
                Err(error_msg)
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}