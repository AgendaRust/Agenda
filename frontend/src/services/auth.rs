// use std::error::Error;

use gloo::net::{
    http::{Request, Response},
    Error,
};
use serde::{Deserialize, Serialize};
// use serde_json::Value

#[derive(Serialize, Deserialize)]
pub struct AuthStruct {
    username: String,
    password: String,
}

impl AuthStruct {
    pub fn new(username: String, password: String) -> Self {
        AuthStruct { username, password }
    }
}

const API_URL: &str = "http://127.0.0.1:8000";

pub async fn login(login_info: &AuthStruct) -> bool {
    let login_url = format!("{API_URL}/login");
    match Request::post(&login_url)
        .json(&login_info)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => response.status() == 200,
        Err(_) => false,
    }
}

pub async fn register(register_info: &AuthStruct) -> Result<Response, Error> {
    let register_url: String = format!("{API_URL}/register");
    let response = Request::post(&register_url)
        .json(&register_info)
        .unwrap()
        .send()
        .await;
    response
}

pub async fn get_notes() -> String {
    let url = "http://127.0.0.1:8000/notes";
    let response = Request::get(&url).send().await.unwrap();
    if response.status() == 200 {
        format!(
            "notes resulted {} {}",
            response.status_text(),
            response.text().await.unwrap_or("aiai".to_string())
        )
    } else {
        format!("notes not found {}", response.status_text())
    }
}
