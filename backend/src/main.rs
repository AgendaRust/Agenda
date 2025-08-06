#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Olá, mundo com Rocket! 🚀"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
