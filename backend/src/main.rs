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
use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true);

    rocket::build()
        .attach(db::init_pool())
        .mount("/", FileServer::from("dist"))
        .mount("/api/", routes::get_auth_routes())
        .mount("/api/notes", routes::get_note_routes())
        .mount("/api/tasks", routes::get_task_routes())
        .mount("/api/reminders", routes::get_reminder_routes())
        .mount("/api/goals", routes::get_goal_routes())
        .mount("/api/reports", routes::get_report_routes())
        .attach(cors.to_cors().unwrap())
}