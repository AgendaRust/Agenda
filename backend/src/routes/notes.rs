use crate::db::{self, Pool};
use crate::dto::CreateNote;
use crate::entity::{notes, prelude::*};
use rocket::{serde::json::Json, State};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

#[post("/", data = "<note_dto>")]
pub async fn create_note(
    db: &State<Pool>,
    note_dto: Json<CreateNote>,
) -> Result<Json<notes::Model>, String> {
    let conn = db as &Pool;

    let new_note = notes::ActiveModel {
        text: Set(note_dto.text.clone()),
        ..Default::default()
    };

    match new_note.insert(conn).await {
        Ok(saved_note) => Ok(Json(saved_note)),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/")]
pub async fn get_all_notes(db: &State<Pool>) -> Result<Json<Vec<notes::Model>>, String> {
    let conn = db.inner();

    match notes::Entity::find().all(conn).await {
        Ok(notes) => Ok(Json(notes)),
        Err(e) => Err(e.to_string()),
    }
}
