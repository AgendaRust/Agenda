use crate::db::Pool;
use crate::dto::authDTO::AuthDto;
use crate::repository::auth_repository::{self, UserError};
use crate::entity::user;
use rocket::State;

pub async fn register_user(
    db: &State<Pool>,
    user_info: &AuthDto,
) -> Result<user::Model, UserError> {
    if !validate_user_fields(user_info) {
        return Err(UserError::InvalidUser("Invalid Fields".to_string()));
    }

    if validate_username_exists(db, user_info).await? {
        return Err(UserError::UserAlreadyExists(
            "Username already been used".to_string(),
        ));
    }

    auth_repository::create_user(db, user_info).await
}

pub async fn login_user(
    db: &State<Pool>,
    user_info: &AuthDto,
) -> Result<user::Model, UserError> {
    auth_repository::find_user_by_credentials(db, user_info).await
}

async fn validate_username_exists(db: &State<Pool>, user_info: &AuthDto) -> Result<bool, UserError> {
    let exist_user = auth_repository::find_by_username(db, &user_info.username).await?;

    match exist_user {
        Some(_) => Ok(true),
        _ => Ok(false),
    }
}

fn validate_user_fields(user_info: &AuthDto) -> bool {
    if user_info.username.len() < 5 || user_info.password.len() < 5 {
        return false;
    }
    true
}