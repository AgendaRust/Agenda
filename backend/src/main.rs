#[macro_use]
extern crate rocket;

mod db;
mod dto;
mod entity;
mod routes;

use dotenvy::dotenv;

use rocket::tokio::time::{sleep, Duration};

#[get("/")]
fn index() -> &'static str {
    "Olá, mundo com Rocket! 🚀"
}

#[get("/delay/<seconds>")]
async fn delay_response(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited after {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(db::init_pool())
        .mount("/", routes![index, delay_response])
        .mount("/notes", routes::get_note_routes())
}
