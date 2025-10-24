use rocket::State;
use crate::db::Pool;
use crate::dto::task_dto::TaskDto;
use crate::dto::task_update_dto::TaskUpdateDto;
use crate::entity::task;
use crate::repository::task_repository::TaskRepository;
use sea_orm::DeleteResult;
use validator::Validate;
/// Enum para erros específicos do serviço de tarefas.
#[allow(dead_code)]
pub enum TaskError {
    TaskNotFound(String),
    DatabaseError(String),
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
    task_info: &TaskUpdateDto,
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