use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    pub dark_mode: bool,
    pub on_toggle: Callback<()>,
}

#[function_component(ThemeToggle)]
pub fn theme_toggle(props: &ThemeToggleProps) -> Html {
    let theme_icon = if props.dark_mode { "☀️" } else { "🌙" };
    let title = if props.dark_mode {
        "Switch to Light Mode"
    } else {
        "Switch to Dark Mode"
    };

    html! {
        <div class="theme-toggle">
            <button
                class="theme-btn"
                onclick={props.on_toggle.reform(|_| ())}
                title={title}
            >
                { theme_icon }
            </button>
        </div>
    }
}
