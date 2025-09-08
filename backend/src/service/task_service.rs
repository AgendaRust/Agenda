use rocket::State;
use crate::db::Pool;
use crate::dto::taskDTO::TaskDto;
use crate::entity::task;
use crate::repository::task_repository::TaskRepository;
use sea_orm::DeleteResult;
use validator::Validate;
use serde_json::{json, Value};
/// Enum para erros específicos do serviço de tarefas.
pub enum TaskError {
    TaskNotFound(String),
    DatabaseError(String),
    Unauthorized(String),
    ValidationError(String),
}

pub async fn get_all_tasks_db(db: &State<Pool>) -> Result<Vec<task::Model>, TaskError> {
    let conn = db.inner();
    let repo = TaskRepository::new(conn);
    repo.find_all()
        .await
        .map_err(|e| TaskError::DatabaseError(e.to_string()))
}

pub async fn get_task_by_id_db(db: &State<Pool>, id: i32) -> Result<task::Model, TaskError> {
    let conn = db.inner();
    let repo = TaskRepository::new(conn);
    repo.find_by_id(id)
        .await
        .map_err(|e| TaskError::DatabaseError(e.to_string()))?
        .ok_or_else(|| TaskError::TaskNotFound(format!("Task with id {} not found", id)))
}

pub async fn get_tasks_by_user_id_db(
    db: &State<Pool>,
    user_id: i32,
) -> Result<Vec<task::Model>, TaskError> {
    let conn = db.inner();
    let repo = TaskRepository::new(conn);
    repo.find_by_user_id(user_id)
        .await
        .map_err(|e| TaskError::DatabaseError(e.to_string()))
}

pub async fn get_task_stats_year_db(
    db: &State<Pool>,
    user_id: i32,
    year: i32,

) -> Result<Value, TaskError> {
    let conn = db.inner();
    let repo = TaskRepository::new(conn);

    let (total_tasks, executed_tasks, percentage, year, most_productive_shift, most_used_category, most_productive_month, most_productive_week) = repo
        .tasks_stats_year(user_id, year)
        .await
        .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

    Ok(json!({
        "total_tasks": total_tasks,
        "executed_tasks": executed_tasks,
        "percentage": format!("{:.2}", percentage),
        "year": year,
        "most_productive_shift": most_productive_shift,
        "most_used_category": most_used_category,
        "most_productive_month": most_productive_month,
        "most_productive_week": most_productive_week
    }))
}

pub async fn get_task_stats_month_db(
    db: &State<Pool>,
    user_id: i32,
    year: i32,
    month: i32,
) -> Result<Value, TaskError> {
    let conn = db.inner();
    let repo = TaskRepository::new(conn);

    let (total_tasks, executed_tasks, percentage, year, month, most_productive_shift, most_used_category, most_productive_week) = repo
        .tasks_stats_month(user_id, year, month)
        .await
        .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

    Ok(json!({
        "total_tasks": total_tasks,
        "executed_tasks": executed_tasks,
        "percentage": format!("{:.2}", percentage),
        "year": year,
        "month": month,
        "most_productive_shift": most_productive_shift,
        "most_used_category": most_used_category,
        "most_productive_week": most_productive_week
    }))
}

pub async fn get_task_stats_week_db(
    db: &State<Pool>,
    user_id: i32,
    year: i32,
    week_num: i32,
) -> Result<Value, TaskError> {
    let conn = db.inner();
    let repo = TaskRepository::new(conn);

    let (total_tasks, executed_tasks, percentage, year, week_num, most_productive_shift, most_used_category, most_productive_day) = repo
        .tasks_stats_week(user_id, year, week_num)
        .await
        .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

    Ok(json!({
        "total_tasks": total_tasks,
        "executed_tasks": executed_tasks,
        "percentage": format!("{:.2}", percentage),
        "year": year,
        "week": week_num,
        "most_productive_shift": most_productive_shift,
        "most_used_category": most_used_category,
        "most_productive_week": most_productive_day
    }))
}
pub async fn register_task_db(
    db: &State<Pool>,
    task_info: &TaskDto,
    user_id: i32,
) -> Result<task::Model, TaskError> {
    task_info.validate().map_err(|e| TaskError::ValidationError(e.to_string()))?; // validator
    let conn = db.inner();
    let repo = TaskRepository::new(conn);
    repo.create_task(task_info, user_id)
        .await
        .map_err(|e| TaskError::DatabaseError(e.to_string()))
}

pub async fn update_task_db(
    db: &State<Pool>,
    id: i32,
    task_info: &TaskDto,
    user_id: i32,
) -> Result<task::Model, TaskError> {
    let conn = db.inner();
    let repo = TaskRepository::new(conn);
    repo.update_task(id, task_info, user_id)
        .await
        .map_err(|e| match e {
            sea_orm::DbErr::RecordNotFound(_) => TaskError::TaskNotFound(e.to_string()),
            _ => TaskError::DatabaseError(e.to_string()),
        })
}

pub async fn update_status_task_db(
    db: &State<Pool>,
    id: i32,
    status: &str,
    user_id: i32,
) -> Result<task::Model, TaskError> {
    let conn = db.inner();
    let repo = TaskRepository::new(conn);
    repo.update_status_task(id, status, user_id)
        .await
        .map_err(|e| match e {
            sea_orm::DbErr::RecordNotFound(_) => TaskError::TaskNotFound(e.to_string()),
            _ => TaskError::DatabaseError(e.to_string()),
        })
}

pub async fn delete_task_db(
    db: &State<Pool>,
    id: i32,
    user_id: i32
) -> Result<DeleteResult, TaskError> {
    let conn = db.inner();
    let repo = TaskRepository::new(conn);
    let result = repo.delete_task(user_id, id)
        .await
        .map_err(|e| TaskError::DatabaseError(e.to_string()))?;

    if result.rows_affected == 0 {
        Err(TaskError::TaskNotFound(format!("Task with id {} not found", id)))
    } else {
        Ok(result)
    }
}