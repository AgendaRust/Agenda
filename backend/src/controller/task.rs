use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::delete;
use rocket::get;
use rocket::post;
use rocket::put;
use rocket::State;
use crate::controller::auth::UserClaim;
use crate::db::Pool;
use crate::dto::task_dto::TaskDto;
use crate::entity::task;
use crate::service::task_service::{delete_task_db, get_all_tasks_db, get_task_by_id_db, register_task_db, update_task_db, get_tasks_by_user_id_db, TaskError};
use crate::dto::task_update_dto::TaskUpdateDto;

#[get("/all")]
pub async fn get_all_tasks(db: &State<Pool>) -> Result<Json<Vec<task::Model>>, (Status, String)> {
    match get_all_tasks_db(db).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(TaskError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        _ => Err((Status::InternalServerError, "Failed to get tasks".to_string())),
    }
}

#[get("/<id>")]
pub async fn get_task_by_id(id: i32, db: &State<Pool>) -> Result<Json<task::Model>, (Status, String)> {
    match get_task_by_id_db(db, id).await {
        Ok(task) => Ok(Json(task)),
        Err(TaskError::TaskNotFound(msg)) => Err((Status::NotFound, msg)),
        Err(TaskError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        _ => Err((Status::InternalServerError, "Failed to get task".to_string())),
    }
}

#[get("/")]
pub async fn get_tasks_by_user_id(
    db: &State<Pool>,
    token: UserClaim,
) -> Result<Json<Vec<task::Model>>, (Status, String)> {
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (Status::BadRequest, "Invalid token: user_id is not valid".to_string())
    })?;

    match get_tasks_by_user_id_db(db, user_id).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(TaskError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        _ => Err((Status::InternalServerError, "Failed to get user tasks".to_string())),
    }
}
#[post("/", data = "<task_dto>")]
pub async fn register_task(
    task_dto: Json<TaskDto>,
    db: &State<Pool>,
    token: UserClaim,
) -> Result<Json<task::Model>, (Status, String)> {
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (Status::BadRequest, "Invalid token: user_id is not valid".to_string())
    })?;

    match register_task_db(db, &task_dto, user_id).await {
        Ok(task) => Ok(Json(task)),
        Err(TaskError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        _ => Err((Status::BadRequest, "Failed to create task".to_string())),
    }
}

#[put("/<id>", data = "<task_dto>")]
pub async fn update_task(
    id: i32,
    task_dto: Json<TaskUpdateDto>,
    db: &State<Pool>,
    token: UserClaim,
) -> Result<Json<task::Model>, (Status, String)> {
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (Status::BadRequest, "Invalid token: user_id is not valid".to_string())
    })?;

    match update_task_db(db, id, &task_dto,user_id).await {
        Ok(task) => Ok(Json(task)),
        Err(TaskError::TaskNotFound(msg)) => Err((Status::NotFound, msg)),
        Err(TaskError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        _ => Err((Status::InternalServerError, "Failed to update task".to_string())),
    }
}

#[delete("/<id>")]
pub async fn delete_task(
    id: i32,
    db: &State<Pool>,
    token: UserClaim,
) -> Result<Status, (Status, String)> {
    let user_id = token.get_id().parse::<i32>().map_err(|_| {
        (Status::BadRequest, "Invalid token: user_id is not valid".to_string())
    })?;

    match delete_task_db(db, id, user_id).await {
        Ok(_) => Ok(Status::NoContent),
        Err(TaskError::TaskNotFound(msg)) => Err((Status::NotFound, msg)),
        Err(TaskError::DatabaseError(msg)) => Err((Status::InternalServerError, msg)),
        _ => Err((Status::InternalServerError, "Failed to delete task".to_string())),
    }
}