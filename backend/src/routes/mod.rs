use crate::controller::task;
use crate::controller::notes;


pub fn get_note_routes() -> Vec<rocket::Route> {
    routes![
        notes::create_note,
        notes::get_all_notes,
        notes::delete_note,
        notes::update_note
    ]
}

pub fn get_task_routes() -> Vec<rocket::Route> {
    routes![
        task::get_all_tasks,
        task::get_task_by_id,
        task::register_task,
        task::update_task,
        task::delete_task,
        task::get_tasks_by_user_id
    ]
}
