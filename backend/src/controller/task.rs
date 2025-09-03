use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use sea_orm::{EntityTrait, Set};
use crate::controller::auth::{ErrorResponse, TokenResponse, UserClaim};
use crate::db::Pool;
use crate::dto::authDTO::AuthDto;
use crate::dto::CreateNote;
use crate::entity::{notes, task};
use crate::service::auth_service;
use crate::repository::auth_repository::UserError;
use crate::dto::taskDTO::TaskDto;
use crate::service::task_service;
use crate::service::task_service::{register_task_db, TaskError};

#[get("/")]
pub async fn get_all_tasks(db: &State<Pool>) -> Result<Json<Vec<task::Model>>, (Status, String)> {
    let conn: &sea_orm::DatabaseConnection = db.inner();

    match task::Entity::find().all(conn).await {
        Ok(notes) => Ok(Json(notes)),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}


#[post("/", data = "<task_dto>")]
pub async fn register_task(
    task_dto: Json<TaskDto>,
    db: &State<Pool>,
    token: UserClaim, // Token decodificado
) -> Result<Json<task::Model>, (Status, String)> {
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (
            Status::BadRequest,
            "Invalid token: user_id is not valid".to_string(),
        )
    })?;

    match register_task_db(db, &task_dto, user_id).await {
        Ok(task) => Ok(Json(task)),
        Err(TaskError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        Err(_) => Err((Status::BadRequest, "Failed to create task".to_string())),
    }
}