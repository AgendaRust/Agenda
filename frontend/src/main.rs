use yew::prelude::*;

#[function_component(Home)]
fn home_component() -> Html {
    let counter = use_state(|| 0);
    let dark_mode = use_state(|| true);

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

    let toggle_theme = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            dark_mode.set(!*dark_mode);
        })
    };

    let theme_class = if *dark_mode {
        "dark-theme"
    } else {
        "light-theme"
    };

    let theme_icon = if *dark_mode { "☀️" } else { "🌙" };

    html! {
        <div class={format!("container {}", theme_class)}>
            <div class="theme-toggle">
                <button
                    class="theme-btn"
                    onclick={toggle_theme}
                    title={if *dark_mode { "Switch to Light Mode" } else { "Switch to Dark Mode" }}
                >
                    { theme_icon }
                </button>
            </div>
            <h1> { "Bem vindo ao nosso projeto de agenda com rust!" } </h1>
            <div class="button-container">
                <button onclick={add_one}>{ "+1" }</button>
                <button onclick={minus_one}>{ "-1" }</button>
            </div>
            <p class="counter">{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<Home>::new().render();
}
