#[macro_use]
extern crate rocket;

mod controller;
mod db;
mod dto;
mod entity;
mod routes;
mod service;
mod repository;

use dotenvy::dotenv;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true);

    rocket::build()
        .attach(db::init_pool())
        .mount("/", routes::get_auth_routes())
        .mount("/notes", routes::get_note_routes())
        .mount("/tasks", routes::get_task_routes())
        .mount("/reminders", routes::get_reminder_routes())
        .mount("/goals", routes::get_goal_routes())
        .mount("/reports", routes::get_report_routes())
        .attach(cors.to_cors().unwrap())
}