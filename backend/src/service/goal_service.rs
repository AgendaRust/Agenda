use crate::db::Pool;
use crate::dto::goalDTO::GoalDto;
use crate::entity::goal;
use rocket::http::Status;
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use sea_orm::prelude::DateTimeUtc;

pub async fn create_goal_db(
    db: &Pool,
    goal_dto: &GoalDto,
    user_id: i32,
) -> Result<goal::Model, (Status, String)> {
    let date_start = goal_dto.date_start.parse::<DateTimeUtc>().map_err(|e| (Status::BadRequest, e.to_string()))?;
    let date_end = goal_dto.date_end.parse::<DateTimeUtc>().map_err(|e| (Status::BadRequest, e.to_string()))?;
    let conn = db;
    let new_goal = goal::ActiveModel {
        user_id: Set(user_id),
        name: Set(goal_dto.name.clone()),
        description: Set(goal_dto.description.clone()),
        category: Set(goal_dto.category.clone()),
        status: Set(goal_dto.status.clone()),
        r#type: Set(goal_dto.r#type.clone()),
        date_start: Set(date_start),
        date_end: Set(date_end),
        ..Default::default()
    };

    match new_goal.insert(conn).await {
        Ok(saved_goal) => Ok(saved_goal),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

pub async fn update_goal_db(
    db: &Pool,
    id: i32,
    goal_dto: &GoalDto,
    user_id: i32,
) -> Result<goal::Model, (Status, String)> {
    let date_start = goal_dto.date_start.parse::<DateTimeUtc>().map_err(|e| (Status::BadRequest, e.to_string()))?;
    let date_end = goal_dto.date_end.parse::<DateTimeUtc>().map_err(|e| (Status::BadRequest, e.to_string()))?;
   let conn = db;
    match goal::Entity::find_by_id(id).one(conn).await {
        Ok(Some(goal_model)) => {
            let updated_goal = goal::ActiveModel {
                id: Set(goal_model.id),
                user_id: Set(user_id),
                name: Set(goal_dto.name.clone()),
                description: Set(goal_dto.description.clone()),
                category: Set(goal_dto.category.clone()),
                status: Set(goal_dto.status.clone()),
                r#type: Set(goal_dto.r#type.clone()),
                date_start: Set(date_start),
                date_end: Set(date_end),
            };
            match updated_goal.update(conn).await {
                Ok(goal) => Ok(goal),
                Err(e) => Err((Status::InternalServerError, e.to_string())),
            }
        }
        Ok(None) => Err((Status::NotFound, "Goal not found".into())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

pub async fn delete_goal_db(
    db: &Pool,
    id: i32,
) -> Result<goal::Model, (Status, String)> {
    let conn = db;
    match goal::Entity::find_by_id(id).one(conn).await {
        Ok(Some(goal_model)) => {
            let deleted_goal = goal_model.clone();
            let active_goal: goal::ActiveModel = goal_model.into();
            match active_goal.delete(conn).await {
                Ok(_) => Ok(deleted_goal),
                Err(e) => Err((Status::InternalServerError, e.to_string())),
            }
        }
        Ok(None) => Err((Status::NotFound, "Goal not found".into())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

pub async fn list_goals_db(
    db: &Pool,
) -> Result<Vec<goal::Model>, (Status, String)> {
    let conn = db;
    match goal::Entity::find().all(conn).await {
        Ok(goals) => Ok(goals),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

pub async fn get_goal_db(
    db: &Pool,
    id: i32,
) -> Result<goal::Model, (Status, String)> {
let conn = db;
    match goal::Entity::find_by_id(id).one(conn).await {
        Ok(Some(goal)) => Ok(goal),
        Ok(None) => Err((Status::NotFound, "Goal not found".into())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}