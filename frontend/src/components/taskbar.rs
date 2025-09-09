use yew::{prelude::*, Properties, Callback};
use gloo_timers::callback::Interval;
use chrono::Local;

#[derive(Properties, PartialEq)]
pub struct TaskbarProps {
    pub on_calendar_toggle: Callback<()>,
    pub calendar_visible: bool,
}

#[function_component(Taskbar)]
pub fn taskbar(props: &TaskbarProps) -> Html {
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
                <button 
                    class={if props.calendar_visible { "taskbar-app-button active" } else { "taskbar-app-button" }}
                    onclick={
                        let on_calendar_toggle = props.on_calendar_toggle.clone();
                        Callback::from(move |_: MouseEvent| {
                            on_calendar_toggle.emit(());
                        })
                    }
                >
                    <span class="app-icon calendar-icon"></span>
                    <span class="app-name">{ "Agenda" }</span>
                </button>
            </div>
            <div class="system-tray">
                <span class="time">{ (*time).clone() }</span>
            </div>
        </div>
    }
}
