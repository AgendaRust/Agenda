use crate::components::{calendar_app::CalendarApp, taskbar::Taskbar};
use yew::prelude::*;

#[function_component(Home)]
pub fn home_component() -> Html {
    html! {
        <div class="app-container">
            <CalendarApp />
            <Taskbar />
        </div>
    }
}
