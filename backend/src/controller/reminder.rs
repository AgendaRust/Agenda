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
use crate::service::reminder_service;



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

#[get("/")]
pub async fn list_reminders(db: &State<Pool>) -> Result<Json<Vec<reminder::Model>>, (Status, String)> {
    match reminder_service::list_reminders_db(db).await {
        Ok(reminders) => Ok(Json(reminders)),
        Err(e) => Err(e),
    }
}

#[get("/<id>")]
pub async fn get_reminder(
    db: &State<Pool>,
    id: i32,
) -> Result<Json<reminder::Model>, (Status, String)> {
    match reminder_service::get_reminder_db(db, id).await {
        Ok(reminder) => Ok(Json(reminder)),
        Err(e) => Err(e),
    }
}

#[put("/<id>", data = "<reminder_dto>")]
pub async fn update_reminder(
    db: &State<Pool>,
    id: i32,
    reminder_dto: Json<reminder_DTO>,
) -> Result<Json<reminder::Model>, (Status, String)> {
    match reminder_service::update_reminder_db(db, id, &reminder_dto).await {
        Ok(reminder) => Ok(Json(reminder)),
        Err(e) => Err(e),
    }
}

#[get("/user")]
pub async fn get_reminders_by_user_id(
    db: &State<Pool>,
    token: UserClaim,
) -> Result<Json<Vec<reminder::Model>>, (Status, String)> {
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (Status::BadRequest, "Invalid token: user_id is not valid".to_string())
    })?;

    match reminder_service::get_reminders_by_user_id_db(db, user_id).await {
        Ok(reminders) => Ok(Json(reminders)),
        Err(ReminderError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        _ => Err((Status::InternalServerError, "Failed to get user reminders".to_string())),
    }
}


// #[post("/")]
// pub async fn register_reminder() -> &'static str {
//     "Hello reminders"
// }