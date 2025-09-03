use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::EntityTrait;

use crate::controller::auth::UserClaim;
use crate::db::Pool;
use crate::dto::reminder_dto::reminder_DTO;
use crate::entity::reminder;
use crate::service::reminder_service::create_reminder_db;
use crate::service::reminder_service::ReminderError;
use crate::service::reminder_service::delete_reminder_db;



#[post("/", data = "<reminder_dto>")]
pub async fn register_reminder(
    reminder_dto: Json<reminder_DTO>,
    db: &State<Pool>,
    token: UserClaim,
) -> Result<Json<reminder::Model>, (Status, String)> {

    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (
            Status::BadRequest,
            "Invalid token: user_id is not valid".to_string(),
        )
    })?;

    match create_reminder_db(db, &reminder_dto, user_id).await {
        Ok(reminder) => Ok(Json(reminder)),
        Err(ReminderError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        Err(_) => Err((Status::BadRequest, "Failed to create reminder".to_string())),
    }
}


/*
#[delete("/<id>")]
pub async fn delete_reminder(id: i32, db: &State<Pool>, token: UserClaim,
) -> Result<Status, (Status, String)> {
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (
            Status::BadRequest,
            "Invalid token: user_id is not valid".to_string(),
        )
    })?;

    match delete_reminder_db(db, id).await {
        Ok(_) => Ok(Status::NoContent),
        Err(ReminderError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        Err(_) => Err((Status::BadRequest, "Failed to delete reminder".to_string())),
    }
}  */


#[delete("/<id>")]
pub async fn delete_reminder(
    db: &State<Pool>,
    id: i32,
) -> Result<Json<reminder::Model>, (Status, String)> {
    match delete_reminder_db(db, id).await {
        Ok(reminder) => Ok(Json(reminder)),
        Err(e) => Err(e),
    }
}  


// #[post("/")]
// pub async fn register_reminder() -> &'static str {
//     "Hello reminders"
// }