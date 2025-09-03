use rocket::State;
use sea_orm::{ActiveModelTrait, Set};
use crate::db::Pool;
use crate::dto::reminder_dto::reminder_DTO;
use crate::entity::reminder;
use rocket::http::Status;

 use sea_orm::EntityTrait;
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

pub async fn delete_reminder_db(db: &Pool, id: i32) -> Result<reminder::Model, (Status, String)> {
    let conn = db;
    match reminder::Entity::find_by_id(id).one(conn).await {
        Ok(Some(reminder_model)) => {
            let deleted_reminder = reminder_model.clone();
            let active_reminder: reminder::ActiveModel = reminder_model.into();
            match active_reminder.delete(conn).await {
                Ok(_) => Ok(deleted_reminder),
                Err(e) => Err((Status::InternalServerError, e.to_string())),
            }
        }
        Ok(None) => Err((Status::NotFound, "Reminder not found".into())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}  


pub async fn list_reminders_db(
    db: &Pool,
) -> Result<Vec<reminder::Model>, (Status, String)> {
    let conn = db;
    match reminder::Entity::find().all(conn).await {
        Ok(reminders) => Ok(reminders),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

pub async fn get_reminder_db(
    db: &Pool,
    id: i32,
) -> Result<reminder::Model, (Status, String)> {
    let conn = db;
    match reminder::Entity::find_by_id(id).one(conn).await {
        Ok(Some(reminder)) => Ok(reminder),
        Ok(None) => Err((Status::NotFound, "Reminder not found".into())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}