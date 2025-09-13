use rocket::routes;
use crate::controller::task;
use crate::controller::notes;
use crate::controller::reminder;
use crate::controller::goal;
use crate::controller::auth;
use crate::controller::report;

pub fn get_auth_routes() -> Vec<rocket::Route> {
    routes![
        auth::login,
        auth::register,
        auth::user_info
    ]
}

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

pub fn get_reminder_routes() -> Vec<rocket::Route> {
    routes![
        reminder::register_reminder,
        reminder::delete_reminder,
        reminder::list_reminders,
        reminder::get_reminder,
        reminder::update_reminder,
        reminder::get_reminders_by_user_id,
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

pub fn get_report_routes() -> Vec<rocket::Route> {
    routes![
        report::get_tasks_stats_year,
        report::get_tasks_stats_month,
        report::get_tasks_stats_week
    ]
}