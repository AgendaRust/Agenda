use crate::db::Pool;
use crate::dto::goal_dto::{GoalDto, GoalResponseDto};
use crate::entity::goal;
use rocket::http::Status;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, ColumnTrait, QueryFilter};
use chrono::{Utc};

fn convert_to_response_dto(goal: goal::Model) -> GoalResponseDto {
    let now = Utc::now();
    let days_remaining = (goal.date_end - now).num_days();
    let total_days = (goal.date_end - goal.date_start).num_days();
    let elapsed_days = (now - goal.date_start).num_days();
    
    let progress_percentage = if total_days > 0 {
        ((elapsed_days as f32 / total_days as f32) * 100.0).min(100.0).max(0.0)
    } else {
        0.0
    };

    GoalResponseDto {
        id: goal.id,
        name: goal.name,
        description: goal.description,
        category: goal.category,
        status: goal.status,
        goal_type: goal.r#type,
        date_start: goal.date_start.format("%Y-%m-%d").to_string(),
        date_end: goal.date_end.format("%Y-%m-%d").to_string(),
        days_remaining,
        progress_percentage,
    }
}

pub async fn create_goal_db(
    db: &Pool,
    goal_dto: &GoalDto,
    user_id: i32,
) -> Result<GoalResponseDto, (Status, String)> {
    // Calcula as datas baseado no tipo de meta
    let (start_date, end_date) = goal_dto.calculate_dates();
    
    // Converte para DateTimeUtc
    let date_start = start_date.and_hms_opt(0, 0, 0)
        .ok_or((Status::BadRequest, "Invalid start date".to_string()))?
        .and_utc();
    
    let date_end = end_date.and_hms_opt(23, 59, 59)
        .ok_or((Status::BadRequest, "Invalid end date".to_string()))?
        .and_utc();

    let conn = db;
    let new_goal = goal::ActiveModel {
        user_id: Set(user_id),
        name: Set(goal_dto.name.clone()),
        description: Set(goal_dto.description.clone()),
        category: Set(goal_dto.category.clone()),
        status: Set(goal_dto.status.clone()),
        r#type: Set(goal_dto.goal_type.clone()),
        date_start: Set(date_start),
        date_end: Set(date_end),
        ..Default::default()
    };

    match new_goal.insert(conn).await {
        Ok(saved_goal) => Ok(convert_to_response_dto(saved_goal)),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

pub async fn update_goal_db(
    db: &Pool,
    id: i32,
    goal_dto: &GoalDto,
    user_id: i32,
) -> Result<GoalResponseDto, (Status, String)> {
    let conn = db;
    
    // Primeiro, busca a meta existente para verificar se pertence ao usuário
    match goal::Entity::find_by_id(id)
        .filter(goal::Column::UserId.eq(user_id))
        .one(conn)
        .await 
    {
        Ok(Some(existing_goal)) => {
            // Calcula as datas baseado no tipo de meta (mantém data de início se não especificada)
            let (start_date, end_date) = if goal_dto.start_date.is_some() {
                goal_dto.calculate_dates()
            } else {
                // Mantém a data de início existente
                let existing_start = existing_goal.date_start.date_naive();
                let temp_dto = GoalDto {
                    name: goal_dto.name.clone(),
                    description: goal_dto.description.clone(),
                    category: goal_dto.category.clone(),
                    status: goal_dto.status.clone(),
                    goal_type: goal_dto.goal_type.clone(),
                    start_date: Some(existing_start.format("%Y-%m-%d").to_string()),
                };
                temp_dto.calculate_dates()
            };
            
            // Converte para DateTimeUtc
            let date_start = start_date.and_hms_opt(0, 0, 0)
                .ok_or((Status::BadRequest, "Invalid start date".to_string()))?
                .and_utc();
            
            let date_end = end_date.and_hms_opt(23, 59, 59)
                .ok_or((Status::BadRequest, "Invalid end date".to_string()))?
                .and_utc();

            let updated_goal = goal::ActiveModel {
                id: Set(existing_goal.id),
                user_id: Set(user_id),
                name: Set(goal_dto.name.clone()),
                description: Set(goal_dto.description.clone()),
                category: Set(goal_dto.category.clone()),
                status: Set(goal_dto.status.clone()),
                r#type: Set(goal_dto.goal_type.clone()),
                date_start: Set(date_start),
                date_end: Set(date_end),
            };
            
            match updated_goal.update(conn).await {
                Ok(goal) => Ok(convert_to_response_dto(goal)),
                Err(e) => Err((Status::InternalServerError, e.to_string())),
            }
        }
        Ok(None) => Err((Status::NotFound, "Goal not found or access denied".into())),
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
) -> Result<Vec<GoalResponseDto>, (Status, String)> {
    let conn = db;
    match goal::Entity::find().all(conn).await {
        Ok(goals) => {
            let goal_responses: Vec<GoalResponseDto> = goals.into_iter()
                .map(convert_to_response_dto)
                .collect();
            Ok(goal_responses)
        },
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

pub async fn get_goal_db(
    db: &Pool,
    id: i32,
) -> Result<GoalResponseDto, (Status, String)> {
    let conn = db;
    match goal::Entity::find_by_id(id).one(conn).await {
        Ok(Some(goal)) => Ok(convert_to_response_dto(goal)),
        Ok(None) => Err((Status::NotFound, "Goal not found".into())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

pub async fn get_goals_by_user_db(
    db: &Pool,
    user_id: i32,
) -> Result<Vec<GoalResponseDto>, (Status, String)> {
    let conn = db;
    match goal::Entity::find()
        .filter(goal::Column::UserId.eq(user_id))
        .all(conn)
        .await 
    {
        Ok(goals) => {
            let goal_responses: Vec<GoalResponseDto> = goals.into_iter()
                .map(convert_to_response_dto)
                .collect();
            Ok(goal_responses)
        },
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}