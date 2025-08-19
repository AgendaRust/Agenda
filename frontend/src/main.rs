// use yew::prelude::*;

mod components;
mod hooks;
mod pages;
mod services;
mod types;
mod utils;

use pages::home::Home;
use pages::login::Login;

fn main() {
    yew::Renderer::<Login>::new().render();
}
