use crate::db::Pool;
use crate::dto::auth::AuthDto;
use crate::entity::prelude::User;
use crate::entity::user;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};

pub enum UserError {
    InvalidUser(String),
    UserAlreadyExists(String),
    DatabaseError(String),
}

pub async fn register_user_db(
    db: &State<Pool>,
    user_info: &AuthDto,
) -> Result<user::Model, UserError> {
    if !validate_user_fields(user_info) {
        return Err(UserError::InvalidUser("Invalid Fields".to_string()));
    }
    if validate_username_exists(db, user_info).await {
        return Err(UserError::UserAlreadyExists(
            "Username already been used".to_string(),
        ));
    }

    let conn = db.inner();

    let new_user = user::ActiveModel {
        username: Set(user_info.username.clone()),
        password: Set(user_info.password.clone()),
        ..Default::default()
    };

    match new_user.insert(conn).await {
        Ok(user) => Ok(user),
        Err(e) => Err(UserError::DatabaseError(e.to_string())),
    }
}

pub async fn login_user_db(
    db: &State<Pool>,
    user_info: &AuthDto,
) -> Result<user::Model, UserError> {
    let conn = db.inner();

    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&user_info.username))
        .one(conn)
        .await;

    let error_msg = "Invalid username or password";
    match user {
        Ok(Some(user)) => {
            if user.password == user_info.password {
                Ok(user)
            } else {
                Err(UserError::InvalidUser(error_msg.to_string()))
            }
        }
        Ok(None) => Err(UserError::InvalidUser(error_msg.to_string())),
        Err(e) => Err(UserError::DatabaseError(e.to_string())),
    }
}

fn validate_user_fields(user_info: &AuthDto) -> bool {
    if user_info.username.len() < 5 || user_info.password.len() < 5 {
        return false;
    }
    true
}

async fn validate_username_exists(db: &State<Pool>, user_info: &AuthDto) -> bool {
    let conn = db.inner();
    let exist_user = user::Entity::find()
        .filter(user::Column::Username.eq(&user_info.username))
        .one(conn)
        .await;

    match exist_user {
        Ok(Some(_)) => true,
        _ => false,
    }
}
