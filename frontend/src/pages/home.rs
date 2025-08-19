use yew::prelude::*;

use crate::components::ThemeToggle;
use crate::hooks::use_theme;

#[function_component(Home)]
pub fn home_component() -> Html {
    let counter = use_state(|| 0);
    let (dark_mode, toggle_theme) = use_theme();

    let add_one = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    let minus_one = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter - 1);
        })
    };

    let reset_counter = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(0);
        })
    };

    let theme_class = if dark_mode {
        "dark-theme"
    } else {
        "light-theme"
    };

    html! {
        <>
            <ThemeToggle dark_mode={dark_mode} on_toggle={toggle_theme} />
            <div class={format!("container {}", theme_class)}>
                <h1> { "Bem vindo ao nosso projeto de agenda com rust!" } </h1>
                <div class="button-container">
                    <button onclick={add_one}>{ "+1" }</button>
                    <button onclick={reset_counter}>{ "Reset" }</button>
                    <button onclick={minus_one}>{ "-1" }</button>
                </div>
                <p class="counter">{ *counter }</p>
            </div>
        </>
    }
}
