use crate::components::{calendar_app::CalendarApp, taskbar::Taskbar};
use yew::{prelude::*, Callback};

#[function_component(Home)]
pub fn home_component() -> Html {
    let calendar_visible = use_state(|| true);

    let toggle_calendar = {
        let calendar_visible = calendar_visible.clone();
        Callback::from(move |_: ()| {
            calendar_visible.set(!*calendar_visible);
        })
    };

    let close_calendar = {
        let calendar_visible = calendar_visible.clone();
        Callback::from(move |_: ()| {
            calendar_visible.set(false);
        })
    };

    html! {
        <div class="app-container">
            <CalendarApp visible={*calendar_visible} on_close={close_calendar} />
            <Taskbar on_calendar_toggle={toggle_calendar} calendar_visible={*calendar_visible} />
        </div>
    }
}
