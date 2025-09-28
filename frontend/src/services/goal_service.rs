use serde::{Deserialize, Serialize};
use gloo::net::http::Request;
use crate::types::goal::Goal;
use super::{API_URL, auth::get_token};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalDto {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub status: String,
    pub goal_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoalUpdateDto {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub status: String,
    pub goal_type: String   
}
pub async fn get_all_goals() -> Result<Vec<Goal>, String> {
    let url = format!("{}/goals/user", API_URL);
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
                match response.json::<Vec<Goal>>().await {
                    Ok(goals) => {
                        web_sys::console::log_1(&format!("Fetched {} goals", goals.len()).into());
                        Ok(goals)
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to parse goals JSON: {}", e);
                        web_sys::console::log_1(&error_msg.clone().into());
                        Err(error_msg)
                    }
                }
            } else {
                let error_msg = format!("Failed to fetch goals: HTTP {}", response.status());
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


pub async fn create_goal(goal_dto: GoalDto) -> Result<Goal, String> {
    let url = format!("{}/goals/", API_URL);
    let token = get_token();
    
    if token.token.is_empty() {
        return Err("No authentication token found".to_string());
    }

    match Request::post(&url)
        .header("Authorization", &format!("Bearer {}", token.token))
        .header("Content-Type", "application/json")
        .json(&goal_dto)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.status() {
            200 | 201 => {
                match response.json::<Goal>().await {
                    Ok(goal) => Ok(goal),
                    Err(e) => {
                        let error_msg = format!("Failed to parse created goal: {}", e);
                        web_sys::console::log_1(&error_msg.clone().into());
                        Err(error_msg)
                    }
                }
            }
            400 => {
                let error_text = response.text().await.unwrap_or_else(|_| "Bad request".to_string());
                Err(format!("Invalid goal data (400): {}", error_text))
            }
            401 => Err("Unauthorized - please login again".to_string()),
            422 => {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown validation error".to_string());
                Err(format!("Validation error (422): {}", error_text))
            },
            _ => {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(format!("HTTP {}: {}", response.status(), error_text))
            },
        },
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

// ...existing code...

pub async fn update_goal(goal_id: i32, goal_dto: GoalDto) -> Result<Goal, String> {
    let url = format!("{}/goals/{}", API_URL, goal_id);
    let token = get_token();
    
    if token.token.is_empty() {
        return Err("No authentication token found".to_string());
    }

    match Request::put(&url)
        .header("Authorization", &format!("Bearer {}", token.token))
        .header("Content-Type", "application/json")
        .json(&goal_dto)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Goal>().await {
                    Ok(goal) => Ok(goal),
                    Err(e) => Err(format!("Failed to parse updated goal: {}", e)),
                }
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Err(format!("Failed to update goal: HTTP {} - {}", response.status(), error_text))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

// pub async fn get_user_goals() -> Result<Vec<Goal>, String> {
//     let url = format!("{}/goals/user", API_URL);
//     let token = get_token();

//     if token.token.is_empty() {
//         return Err("No authentication token found".to_string());
//     }

//     match Request::get(&url)
//         .header("Authorization", &format!("Bearer {}", token.token))
//         .send()
//         .await
//     {
//         Ok(response) => {
//             if response.status() == 200 {
//                 match response.json::<Vec<Goal>>().await {
//                     Ok(goals) => Ok(goals),
//                     Err(e) => Err(format!("Failed to parse goals: {}", e)),
//                 }
//             } else {
//                 Err(format!("Failed to fetch goals: HTTP {}", response.status()))
//             }
//         }
//         Err(e) => Err(format!("Network error: {}", e)),
//     }
// }

pub async fn delete_goal(id: i32) -> Result<(), String> {
    let url = format!("{}/goals/{}", API_URL, id);
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
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                let error_msg = format!("Failed to delete goal: HTTP {} - {}", response.status(), error_text);
                web_sys::console::log_1(&error_msg.clone().into());
                Err(error_msg)
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}