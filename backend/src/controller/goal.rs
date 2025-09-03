use crate::db::Pool;
use crate::dto::goalDTO::GoalDto;
use crate::entity::goal;
use crate::service::goal_service;
use rocket::http::Status;
use rocket::{serde::json::Json, State};

#[post("/", data = "<goal_dto>")]
pub async fn create_goal(
    db: &State<Pool>,
    goal_dto: Json<GoalDto>,
) -> Result<Json<goal::Model>, (Status, String)> {
    match goal_service::create_goal_db(db, &goal_dto).await {
        Ok(goal) => Ok(Json(goal)),
        Err(e) => Err(e),
    }
}

#[put("/<id>", data = "<goal_dto>")]
pub async fn update_goal(
    db: &State<Pool>,
    id: i32,
    goal_dto: Json<GoalDto>,
) -> Result<Json<goal::Model>, (Status, String)> {
    match goal_service::update_goal_db(db, id, &goal_dto).await {
        Ok(goal) => Ok(Json(goal)),
        Err(e) => Err(e),
    }
}

#[delete("/<id>")]
pub async fn delete_goal(
    db: &State<Pool>,
    id: i32,
) -> Result<Json<goal::Model>, (Status, String)> {
    match goal_service::delete_goal_db(db, id).await {
        Ok(goal) => Ok(Json(goal)),
        Err(e) => Err(e),
    }
}

#[get("/")]
pub async fn list_goals(db: &State<Pool>) -> Result<Json<Vec<goal::Model>>, (Status, String)> {
    match goal_service::list_goals_db(db).await {
        Ok(goals) => Ok(Json(goals)),
        Err(e) => Err(e),
    }
}

#[get("/<id>")]
pub async fn get_goal(
    db: &State<Pool>,
    id: i32,
) -> Result<Json<goal::Model>, (Status, String)> {
    match goal_service::get_goal_db(db, id).await {
        Ok(goal) => Ok(Json(goal)),
        Err(e) => Err(e),
    }
}