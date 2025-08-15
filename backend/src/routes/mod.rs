mod notes;

pub fn get_note_routes() -> Vec<rocket::Route> {
    routes![
        notes::create_note,
        notes::get_all_notes,
        notes::delete_note,
        notes::update_note
    ]
}
