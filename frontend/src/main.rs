// use yew::prelude::*;

mod components;
mod hooks;
mod pages;
mod services;
mod types;
mod utils;

use utils::routes::Main;

fn main() {
    yew::Renderer::<Main>::new().render();
}
