use yew::prelude::*;
use crate::components::calendar_app::CalendarApp;



#[function_component(Home)]
pub fn home_component() -> Html {
    html! {
        <>
            <CalendarApp />
        </>
    }
}
