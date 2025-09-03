use crate::db::Pool;
use crate::dto::CreateNote;
use crate::entity::notes;
use rocket::http::Status;
use rocket::{serde::json::Json, State};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

#[post("/", data = "<note_dto>")]
pub async fn create_note(
    db: &State<Pool>,
    note_dto: Json<CreateNote>,
) -> Result<Json<notes::Model>, (Status, String)> {
    let conn = db.inner();

    let new_note = notes::ActiveModel {
        text: Set(note_dto.text.clone()),
        bolsonar: Set(note_dto.bolsonar.clone()),
        ..Default::default()
    };

    match new_note.insert(conn).await {
        Ok(saved_note) => Ok(Json(saved_note)),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

#[put("/<id>", data = "<note_dto>")]
pub async fn update_note(
    db: &State<Pool>,
    id: i32,
    note_dto: Json<CreateNote>,
) -> Result<Json<notes::Model>, (Status, String)> {
    let conn = db.inner();

    match notes::Entity::find_by_id(id).one(conn).await {
        Ok(Some(note)) => {
            let updated_note = notes::ActiveModel {
                id: Set(note.id),
                text: Set(note_dto.text.clone()),
                bolsonar: Set(note_dto.bolsonar.clone()),
                ..Default::default()
            };

            match updated_note.update(conn).await {
                Ok(updated_note) => Ok(Json(updated_note)),
                Err(e) => Err((Status::InternalServerError, e.to_string())),
            }
        }
        Ok(None) => Err((Status::NotFound, "Note not found".into())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

#[delete("/<id>")]
pub async fn delete_note(
    db: &State<Pool>,
    id: i32,
) -> Result<Json<notes::Model>, (Status, String)> {
    let conn = db.inner();

    match notes::Entity::find_by_id(id).one(conn).await {
        Ok(Some(note)) => {
            let deleted_note = note.clone();
            let active_note: notes::ActiveModel = note.into();

            match active_note.delete(conn).await {
                Ok(_) => Ok(Json(deleted_note)),
                Err(e) => Err((Status::InternalServerError, e.to_string())),
            }
        }
        Ok(None) => Err((Status::NotFound, "Note not found".into())),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}

#[get("/")]
pub async fn get_all_notes(db: &State<Pool>) -> Result<Json<Vec<notes::Model>>, (Status, String)> {
    let conn: &sea_orm::DatabaseConnection = db.inner();

    match notes::Entity::find().all(conn).await {
        Ok(notes) => Ok(Json(notes)),
        Err(e) => Err((Status::InternalServerError, e.to_string())),
    }
}
