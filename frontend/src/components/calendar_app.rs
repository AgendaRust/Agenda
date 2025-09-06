use yew::{function_component, html, Html, use_state, Callback, MouseEvent};
use chrono::{Local, NaiveDate};

use crate::components::{task_card::TaskCard, task_form::TaskForm};
use crate::types::TaskDuration;


#[function_component(CalendarApp)]
pub fn calendar_app() -> Html {
    let show_task_form = use_state(|| false);
    
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
                    { for (1..=31).map(|day| html! {
                        <span class="unique-day">{ day }</span>
                    }) }
                    <span class="current-day"> { "32" } </span>
                </div>
            </div>
            
            <div class="sidebar">
                <div class="sidebar-header">
                    <h3>{ "Tasks" }</h3>
                </div>
                <div class="task-list">
                    <TaskCard 
                        title={"Meeting with Team".to_string()}
                        category={"Work".to_string()}
                        description={"Weekly team sync to discuss project progress and upcoming deadlines".to_string()}
                        date={"May 20, 2023".to_string()}
                        time={"10:00".to_string()}
                        duration={TaskDuration::UmaHora}
                    />
                    <TaskCard 
                        title={"Project Review".to_string()}
                        category={"Work".to_string()}
                        description={"Review project deliverables and prepare for client presentation".to_string()}
                        date={"May 21, 2023".to_string()}
                        time={"14:30".to_string()}
                        duration={TaskDuration::MeiaHora}
                    />
                    <TaskCard 
                        title={"Morning Workout".to_string()}
                        category={"Personal".to_string()}
                        description={"Daily exercise routine and stretching".to_string()}
                        date={"May 22, 2023".to_string()}
                        time={"07:00".to_string()}
                        duration={TaskDuration::Manha}
                    />
                    <TaskCard 
                        title={"Lunch Meeting".to_string()}
                        category={"Business".to_string()}
                        description={"Client meeting to discuss new project requirements and timeline".to_string()}
                        date={"May 23, 2023".to_string()}
                        time={"12:00".to_string()}
                        duration={TaskDuration::Tarde}
                    />
                    <TaskCard 
                        title={"Code Review".to_string()}
                        category={"Work".to_string()}
                        description={"Review pull requests and provide feedback to team members".to_string()}
                        date={"May 24, 2023".to_string()}
                        time={"15:00".to_string()}
                        duration={TaskDuration::UmaHora}
                    />
                </div>
            </div>
            
            // TaskForm as overlay/modal
            <TaskForm 
                visible={*show_task_form} 
                on_close={close_task_form} 
                selected_date={*selected_date}
            />
        </div>
    }
}