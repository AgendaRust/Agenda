use rocket::State;
use sea_orm::{ActiveModelTrait, Set};
use crate::db::Pool;
use crate::dto::reminder_dto::reminder_DTO;
use crate::entity::reminder;

/// Enum para erros específicos do serviço de reminders
pub enum ReminderError {
    ReminderNotFound(String),
    DatabaseError(String),
    Unauthorized(String),
}

/// Função que cria um novo reminder no banco
pub async fn create_reminder_db(
    db: &State<Pool>,
    reminder_info: &reminder_DTO,
    user_id: i32, // Recebe o user_id diretamente
) -> Result<reminder::Model, ReminderError> {
    let conn = db.inner();
    
    let new_reminder = reminder::ActiveModel {
        name: Set(reminder_info.name.clone()),
        user_id: Set(user_id), // Usa o user_id recebido
        category: Set(reminder_info.category.clone()),
        date_end: Set(reminder_info.date_end.clone()),
        ..Default::default() // id será gerado automaticamente
    };

    match new_reminder.insert(conn).await {
        Ok(reminder) => Ok(reminder),
        Err(e) => Err(ReminderError::DatabaseError(e.to_string())),
    }
}
