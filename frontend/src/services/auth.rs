use gloo::net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use js_sys::Date;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthStruct {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenClaims {
    pub exp: u64,
    pub iat: u64,
    pub user: UserData,
}

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
            400 => LoginResult::IncorrectCredentials,
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
            400 => RegisterResult::InvalidFields,
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

pub fn verify_token(token: &Token) -> bool {
    // tem que verificar sem checar a secret key
    // apenas para saber se está valido a data
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.insecure_disable_signature_validation();
    match decode::<TokenClaims>(&token.token, &key, &validation) {
        Ok(data) => {
            let user_id = data.claims.user.id;
            let exp_timestamp = data.claims.exp;
            let current_timestamp = (Date::now() / 1000.0) as u64;

            if exp_timestamp < current_timestamp {
                web_sys::console::log_1(&format!("expired token").into());
                return false;
            }
            web_sys::console::log_1(&format!("valid token, user id: {user_id}").into());
            true
        }
        Err(error) => {
            web_sys::console::log_1(&format!("error decoding data: {error:?}").into());
            false
        }
    }
}
