use crate::controller::task;
use crate::controller::notes;
use crate::controller::goal;

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

pub fn get_goal_routes() -> Vec<rocket::Route> {
    routes![
        goal::create_goal,
        goal::list_goals,
        goal::get_goal,
        goal::update_goal,
        goal::delete_goal
    ]
}