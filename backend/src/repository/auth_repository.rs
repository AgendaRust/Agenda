use crate::db::Pool;
use crate::dto::authDTO::AuthDto;
use crate::entity::prelude::*;
use crate::entity::user;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rocket::State;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use crate::entity::user::Model;

pub enum UserError {
    InvalidUser(String),
    UserAlreadyExists(String),
    DatabaseError(String),
}

pub async fn create_user(
    db: &State<Pool>,
    user_info: &AuthDto,
) -> Result<user::Model, UserError> {

    let conn = db.inner();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user_info.password.as_bytes(), &salt)
        .map_err(|e| UserError::DatabaseError(e.to_string()))?
        .to_string();
    let new_user = user::ActiveModel {
        username: Set(user_info.username.clone()),
        password: Set(password_hash),
        ..Default::default()
    };

    match new_user.insert(conn).await {
        Ok(user) => Ok(user),
        Err(e) => Err(UserError::DatabaseError(e.to_string())),
    }
}

pub async fn find_user_by_credentials(
    db: &State<Pool>,
    user_info: &AuthDto,
) -> Result<user::Model, UserError> {
    let conn = db.inner();

    let user = User::find()
        .filter(user::Column::Username.eq(&user_info.username))
        .one(conn)
        .await;

    let error_msg = "Invalid username or password";
    let argon2 = Argon2::default();

    match user {
        Ok(Some(user)) => {
            let parsed_hash = PasswordHash::new(&user.password)
                .map_err(|e| UserError::DatabaseError(e.to_string()))?;
            if argon2
                .verify_password(user_info.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                Ok(user)
            } else {
                Err(UserError::InvalidUser(error_msg.to_string()))
            }
        }
        Ok(None) => Err(UserError::InvalidUser(error_msg.to_string())),
        Err(e) => Err(UserError::DatabaseError(e.to_string())),
    }
}

pub async fn find_by_username(db: &State<Pool>, username: &str) -> Result<Option<Model>, UserError> {
    let conn = db.inner();
    let user = User::find()
        .filter(user::Column::Username.eq(username))
        .one(conn)
        .await
        .map_err(|e| UserError::DatabaseError(e.to_string()))?;

    Ok(user)
}

