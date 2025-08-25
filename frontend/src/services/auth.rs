// use std::error::Error;

use gloo::net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
// use serde_json::Value

#[derive(Serialize, Deserialize)]
pub struct AuthStruct {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

// pub struct TokenClaims {
//     pub exp: usize,
//     pub iat: usize,
//     pub id: String,
// }

impl AuthStruct {
    pub fn new(username: String, password: String) -> Self {
        AuthStruct { username, password }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token {
            token: "".to_string(),
        }
    }
}

pub enum LoginResult {
    Success,
    IncorrectCredentials,
    NetworkError,
}

pub enum RegisterResult {
    Success,
    InvalidFields,
    NetworkError,
}

const API_URL: &str = "http://127.0.0.1:8000";

pub async fn login(login_info: &AuthStruct) -> LoginResult {
    let login_url = format!("{API_URL}/login");
    match Request::post(&login_url)
        .json(&login_info)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.status() {
            200 | 201 => {
                let token_value = response.json::<Token>().await.unwrap();
                save_token(token_value.token);
                LoginResult::Success
            }
            401 => LoginResult::IncorrectCredentials,
            _ => LoginResult::NetworkError,
        },
        Err(_) => LoginResult::NetworkError,
    }
}

pub async fn register(register_info: &AuthStruct) -> RegisterResult {
    let register_url: String = format!("{API_URL}/register");
    match Request::post(&register_url)
        .json(&register_info)
        .unwrap()
        .send()
        .await
    {
        Ok(response) => match response.status() {
            200 | 201 => {
                let token_value = response.json::<Token>().await.unwrap();
                save_token(token_value.token);
                RegisterResult::Success
            }
            4001 => RegisterResult::InvalidFields,
            _ => RegisterResult::NetworkError,
        },
        Err(_) => RegisterResult::NetworkError,
    }
}

pub fn save_token(token_value: String) {
    let token = Token { token: token_value };
    web_sys::console::log_1(&format!("saving token {}", token.token).into());
    LocalStorage::set("token", token).unwrap();
}

pub fn get_token() -> Token {
    LocalStorage::get::<Token>("token").unwrap_or_default()
}

// pub async fn get_notes() -> String {
//     let url = "http://127.0.0.1:8000/notes";
//     let response = Request::get(&url).send().await.unwrap();
//     if response.status() == 200 {
//         format!(
//             "notes resulted {} {}",
//             response.status_text(),
//             response.text().await.unwrap_or("aiai".to_string())
//         )
//     } else {
//         format!("notes not found {}", response.status_text())
//     }
// }
