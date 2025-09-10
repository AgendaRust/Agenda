use chrono::{DateTime, Utc};
use gloo::net::http::Request;
use serde::{Serialize, Deserialize};
use crate::types::reminder::Reminder;

use super::{API_URL, auth::get_token};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReminderDto {
    pub name: String,
    pub category: String,
    pub date_end: DateTime<Utc>,
}

pub enum ReminderResult {
    Success(Reminder),
    InvalidFields,
    NetworkError(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReminderUpdateDto {
    pub name: String,
    pub category: String,
}

pub async fn get_all_reminders() -> Result<Vec<Reminder>, String> {
    let url = format!("{}/reminders/user", API_URL);
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
                let reminders: Vec<Reminder> = response.json().await.unwrap_or_else(|_| vec![]);
                web_sys::console::log_1(&format!("Fetched {} reminders", reminders.len()).into());
                Ok(reminders)
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(format!("Failed to fetch reminders: HTTP {} - {}", response.status(), error_text))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn create_reminder(reminder_info: &ReminderDto) -> ReminderResult {
    let url = format!("{}/reminders", API_URL);
    let token = get_token();

    // Check if we have a valid token
    if token.token.is_empty() {
        return ReminderResult::NetworkError("No authentication token found".into());
    }

    match Request::post(&url)
        .header("Authorization", &format!("Bearer {}", token.token))
        .header("Content-Type", "application/json")
        .json(&reminder_info)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.status() {
            200 | 201 => {
                let reminder: Reminder = response.json().await.unwrap();
                ReminderResult::Success(reminder)
            }
            400 => ReminderResult::InvalidFields,
            401 => ReminderResult::NetworkError("Unauthorized - please login again".into()),
            422 => {
                // Try to get the response body for more details
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown validation error".to_string());
                ReminderResult::NetworkError(format!("Validation error (422): {}", error_text))
            },
            _ => {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                ReminderResult::NetworkError(format!("HTTP {}: {}", response.status(), error_text))
            },
        },
        Err(e) => ReminderResult::NetworkError(e.to_string()),
    }
}

pub async fn delete_reminder(reminder_id: u32) -> Result<(), String> {
    let url = format!("{}/reminders/{}", API_URL, reminder_id);
    let token = get_token();

    if token.token.is_empty() {
        return Err("No authentication token found".into());
    }

    match Request::delete(&url)
        .header("Authorization", &format!("Bearer {}", token.token))
        .send()
        .await
    {
        Ok(response) => match response.status() {
            200 | 204 => Ok(()),
            401 => Err("Unauthorized - please login again".into()),
            404 => Err("Reminder not found".into()),
            _ => {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(format!("HTTP {}: {}", response.status(), error_text))
            },
        },
        Err(e) => Err(e.to_string()),
    }
}

pub async fn update_reminder(reminder_id: u32, dto: ReminderUpdateDto) -> Result<(), String> {
    let url = format!("{}/reminders/{}", API_URL, reminder_id);
    let token = get_token();
    if token.token.is_empty() {
        return Err("No authentication token found".to_string());
    }

    match Request::put(&url)
        .header("Authorization", &format!("Bearer {}", token.token))
        .header("Content-Type", "application/json")
        .json(&dto)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 || response.status() == 204 {
                Ok(())
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                let error_msg = format!("Failed to update reminder: HTTP {} - {}", response.status(), error_text);
                web_sys::console::log_1(&error_msg.clone().into());
                Err(error_msg)
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}
