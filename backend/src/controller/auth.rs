use crate::db::Pool;
use crate::dto::auth::{self, AuthDto};
use crate::entity::user;
// use crate::entity::user;
use crate::service::auth_service::{self, UserError};
use rocket::http::Status;
use rocket::{serde::json::Json, State};
// use sea_orm::{ActiveModelTrait, EntityTrait, Set};
// #[macro_use]
// extern crate rocket;

use rocket_jwt::jwt;

static SECRET_KEY: &str = "secret";

#[jwt(SECRET_KEY, exp = 3600)]
pub struct UserClaim {
    id: String,
}

#[post("/register", data = "<auth_dto>")]
pub async fn register(auth_dto: Json<auth::AuthDto>, db: &State<Pool>) -> (Status, String) {
    let result = auth_service::register_user_db(db, &auth_dto).await;
    match result {
        Ok(user) => {
            let user_claim = UserClaim {
                id: user.id.to_string(),
            };
            let token = UserClaim::sign(user_claim);
            (Status::Created, token)
        }
        Err(err) => {
            let (status, message) = match err {
                UserError::InvalidUser(msg) => (Status::BadRequest, msg),
                UserError::UserAlreadyExists(msg) => (Status::Conflict, msg),
                UserError::DatabaseError(msg) => (Status::InternalServerError, msg),
            };
            (status, message)
        }
    }
}

#[post("/login", data = "<auth_dto>")]
pub async fn login(
    db: &State<Pool>,
    auth_dto: Json<auth::AuthDto>,
) -> Result<(Status, String), String> {
    let result = auth_service::login_user_db(db, &auth_dto).await;
    match result {
        Ok(user) => {
            let user_claim = UserClaim {
                id: user.id.to_string(),
            };
            let token = UserClaim::sign(user_claim);
            Ok((Status::Ok, token))
        }
        Err(err) => {
            let message = match err {
                UserError::InvalidUser(msg) | UserError::UserAlreadyExists(msg) => {
                    (Status::BadRequest, msg)
                }
                UserError::DatabaseError(msg) => (Status::InternalServerError, msg),
            };
            Err(message.1)
        }
    }
}

#[get("/user_info")]
pub async fn user_info(user_claim: UserClaim) -> String {
    format!("User id: {}", user_claim.id)
}
