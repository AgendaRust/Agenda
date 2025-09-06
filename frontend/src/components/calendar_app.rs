use yew::{function_component, html, Html, use_state, Callback, MouseEvent};
use chrono::{Local, NaiveDate};

use crate::components::task_form::TaskForm;


#[function_component(CalendarApp)]
pub fn calendar_app() -> Html {
    let show_task_form = use_state(|| true);
    
    // Date state - using placeholder values for now
    let selected_date = use_state(|| {
        // Placeholder: October 15, 2023
        NaiveDate::from_ymd_opt(2023, 10, 15).unwrap_or_else(|| Local::now().date_naive())
    });

    let toggle_task_form = {
        let show_task_form = show_task_form.clone();
        Callback::from(move |_: MouseEvent| {
            show_task_form.set(!*show_task_form);
        })
    };

    let close_task_form = {
        let show_task_form = show_task_form.clone();
        Callback::from(move |_: ()| {
            show_task_form.set(false);
        })
    };
    
    
    
    html! {
        <div class="calendar-app">
            <div class="calendar">
                <h2 class="calendar-heading">{ "Calendar" }</h2>
                <div class="navigate-date">
                    <h2 class="month"> { "October" } </h2>
                    <h2 class="year"> { "2023" } </h2>
                    <div class="calendar-buttons">
                        <button>{ "<" }</button>
                        <button>{ ">" }</button>
                        <button onclick={toggle_task_form}>{ "+" }</button>
                    </div>
                    // Calendar grid implementation goes here
                </div>
                <div class="weekdays">
                    <span class="weekday">{ "Sun" }</span>
                    <span class="weekday">{ "Mon" }</span>
                    <span class="weekday">{ "Tue" }</span>
                    <span class="weekday">{ "Wed" }</span>
                    <span class="weekday">{ "Thu" }</span>
                    <span class="weekday">{ "Fri" }</span>
                    <span class="weekday">{ "Sat" }</span>
                </div>
                <div class="days">
                    // Days of the month will be rendered here
                    { for (1..=31).map(|day| html! {
                        
                            <span class="unique-day">{ day }</span>
                            // Tasks for the day can be listed here
                        
                    }) }
                    <span class="current-day"> { "32" } </span>
                </div>
                <div class="events">
                    <TaskForm 
                        visible={*show_task_form} 
                        on_close={close_task_form} 
                        selected_date={*selected_date}
                    />
                    
                    <div class="task">
                        <div class="task-date-wrapper">
                            <div class="task-date"> { "May 20, 2023" } </div>
                            <div class="task-time"> { "10:00 - 11:00" } </div>
                        </div>
                        <div class="task-title"> { "Meeting with Team" } </div>
                        <div class="task-buttons">
                            <button class="task-edit"> { "Edit" } </button>
                            <button class="task-delete"> { "Delete" } </button>
                        </div>
                    </div>
                </div>
                
            </div>
        </div>
    }
}