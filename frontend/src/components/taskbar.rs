use yew::prelude::*;
use gloo_timers::callback::Interval;
use chrono::Local;

#[function_component(Taskbar)]
pub fn taskbar() -> Html {
    let time = use_state(|| Local::now().format("%I:%M %p").to_string());

    {
        let time = time.clone();
        use_effect(move || {
            let interval = Interval::new(1000, move || {
                time.set(Local::now().format("%I:%M %p").to_string());
            });
            
            let a = || drop(interval);
            a
        });
    }

    html! {
        <div class="taskbar">
            <button class="start-button"></button>
            <div class="taskbar-divider"></div>
            <div class="quick-launch">
                // Icons for quick launch can be added here
            </div>
            <div class="taskbar-main">
                // Open application tabs will go here
            </div>
            <div class="system-tray">
                <span class="time">{ (*time).clone() }</span>
            </div>
        </div>
    }
}
