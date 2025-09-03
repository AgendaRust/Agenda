use crate::controller::task;
use crate::controller::notes;
use crate::controller::reminder;

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
        task::register_task
    ]
}
pub fn get_reminder_routes() -> Vec<rocket::Route> {
    routes![
        reminder::register_reminder,
        reminder::delete_reminder,
        reminder::list_reminders,
        reminder::get_reminder,
        reminder::update_reminder
    ]
}