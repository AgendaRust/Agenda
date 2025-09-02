use rocket::State;
use sea_orm::{ActiveModelTrait, Set};
use crate::db::Pool;
use crate::dto::taskDTO::TaskDto;
use crate::entity::task;

/// Enum para erros específicos do serviço de tarefas.
pub enum TaskError {
    TaskNotFound(String),
    DatabaseError(String),
    Unauthorized(String),
}
pub async fn register_task_db(
    db: &State<Pool>,
    task_info: &TaskDto,
    user_id: i32, // Recebe o user_id diretamente
) -> Result<task::Model, TaskError> {
    let conn = db.inner();
    let new_task = task::ActiveModel {
        title: Set(task_info.title.clone()),
        user_id: Set(user_id), // Usa o user_id recebido
        description: Set(task_info.description.clone()),
        status: Set(task_info.status.clone()),
        begin_date: Set(task_info.begin_date),
        complete_date: Set(task_info.complete_date),
        category: Set(task_info.category.clone()),
        r#type: Set(task_info.r#type.clone()),
        ..Default::default()
    };

    match new_task.insert(conn).await {
        Ok(task) => Ok(task),
        Err(e) => Err(TaskError::DatabaseError(e.to_string())),
    }
}