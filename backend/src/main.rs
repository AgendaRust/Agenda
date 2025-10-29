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
use rocket::fs::{FileServer, NamedFile};
use rocket::http::Status;
use std::path::{Path, PathBuf};

#[get("/<_path..>", rank = 11)]
async fn spa_fallback(_path: PathBuf) -> Result<NamedFile, Status> {
    NamedFile::open("dist/index.html")
        .await
        .map_err(|_| Status::NotFound)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true);

    let mut rocket = rocket::build()
        .attach(db::init_pool())
        .mount("/api/", routes::get_auth_routes())
        .mount("/api/notes", routes::get_note_routes())
        .mount("/api/tasks", routes::get_task_routes())
        .mount("/api/reminders", routes::get_reminder_routes())
        .mount("/api/goals", routes::get_goal_routes())
        .mount("/api/reports", routes::get_report_routes())
        .attach(cors.to_cors().unwrap());

    if Path::new("dist").exists() {
        rocket = rocket
            .mount("/", FileServer::from("dist"))
            .mount("/", routes![spa_fallback]);
    }

    rocket
}
