use crate::db::Pool;
use crate::dto::authDTO::AuthDto;
// use crate::entity::user;
// use crate::entity::user;
use crate::repository::auth_repository:: UserError;
use crate::repository::auth_repository;
use rocket::http::Status;
use rocket::{serde::json::Json, State};
// use sea_orm::{ActiveModelTrait, EntityTrait, Set};
// #[macro_use]
// extern crate rocket;
use rocket_jwt::jwt;
use std::env::{var, VarError};
use crate::service::auth_service;

#[derive(serde::Serialize)]
pub struct TokenResponse {
    token: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    error: String,
}

fn get_secret_key() -> Result<String, VarError> {
    var("JWT_SECRET_KEY")
}

#[jwt(get_secret_key(), exp = 36000)]
pub struct UserClaim {
    id: String,
}

impl UserClaim {
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

#[post("/register", data = "<auth_dto>")]
pub async fn register(
    auth_dto: Json<AuthDto>,
    db: &State<Pool>,
) -> Result<(Status, Json<TokenResponse>), (Status, Json<ErrorResponse>)> {
    let result: Result<crate::entity::user::Model, UserError> =
        auth_repository::create_user(db, &auth_dto).await;
    match result {
        Ok(user) => {
            let user_claim = UserClaim {
                id: user.id.to_string(),
            };
            let token = UserClaim::sign(user_claim);
            Ok((Status::Created, Json(TokenResponse { token })))
        }
        Err(err) => match err {
            UserError::InvalidUser(msg) => {
                Err((Status::BadRequest, Json(ErrorResponse { error: msg })))
            }
            UserError::UserAlreadyExists(msg) => {
                Err((Status::Conflict, Json(ErrorResponse { error: msg })))
            }
            UserError::DatabaseError(msg) => Err((
                Status::InternalServerError,
                Json(ErrorResponse { error: msg }),
            )),
        },
    }
}

#[post("/login", data = "<auth_dto>")]
pub async fn login(
    db: &State<Pool>,
    auth_dto: Json<AuthDto>,
) -> Result<Json<TokenResponse>, (Status, Json<ErrorResponse>)> {
    let result = auth_service::login_user(db, &auth_dto).await;

    println!(
        "secret key {}",
        get_secret_key().unwrap_or_else(|_| "secret".into())
    );
    match result {
        Ok(user) => {
            let user_claim = UserClaim {
                id: user.id.to_string(),
            };
            let token = UserClaim::sign(user_claim);
            Ok(Json(TokenResponse { token }))
        }
        Err(err) => match err {
            UserError::InvalidUser(msg) | UserError::UserAlreadyExists(msg) => {
                Err((Status::BadRequest, Json(ErrorResponse { error: msg })))
            }
            UserError::DatabaseError(msg) => Err((
                Status::InternalServerError,
                Json(ErrorResponse { error: msg }),
            )),
        },
    }
}

#[get("/user_info")]
pub async fn user_info(user_claim: UserClaim) -> String {
    format!("User id: {}", user_claim.id)
}
