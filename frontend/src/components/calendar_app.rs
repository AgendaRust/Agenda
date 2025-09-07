use yew::{function_component, html, Html, use_state, Callback, MouseEvent};
use chrono::{Local, NaiveDate, Datelike};

use crate::components::{task_card::TaskCard, task_form::TaskForm};
use crate::types::TaskDuration;


#[function_component(CalendarApp)]
pub fn calendar_app() -> Html {
    let show_task_form = use_state(|| false);
    
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

    let current_date = Local::now().date_naive();

    let current_month = use_state(|| current_date.month());
    let current_year = use_state(|| current_date.year());
    let selected_day = use_state(|| current_date.day()); // Track selected day

    let on_day_click = {
        let selected_day = selected_day.clone();
        Callback::from(move |day: u32| {
            selected_day.set(day);
        })
    };

    let prev_month = {
        let current_month = current_month.clone();
        let current_year = current_year.clone();
        Callback::from(move |_: MouseEvent| {
            if *current_month == 1 {
                current_month.set(12);
                current_year.set(*current_year - 1);
            } else {
                current_month.set(*current_month - 1);
            }
        })
    };

    let next_month = {
        let current_month = current_month.clone();
        let current_year = current_year.clone();
        Callback::from(move |_: MouseEvent| {
            if *current_month == 12 {
                current_month.set(1);
                current_year.set(*current_year + 1);
            } else {
                current_month.set(*current_month + 1);
            }
        })
    };

    let days_of_week = ["Dom", "Seg", "Ter", "Qua", "Qui", "Sex", "Sáb"];
    let months_of_year = [
        "Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho",
        "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"
    ];
    
    let days_in_month = match *current_month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (*current_year % 4 == 0 && *current_year % 100 != 0) || (*current_year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    };

    let first_day_of_month = NaiveDate::from_ymd_opt(*current_year, *current_month, 1)
        .unwrap_or_else(|| Local::now().date_naive());
    
    let first_weekday = first_day_of_month.weekday().number_from_sunday() as usize - 1;
    
    let total_cells_used = first_weekday + days_in_month as usize;
    let remaining_cells = if total_cells_used % 7 == 0 { 0 } else { 7 - (total_cells_used % 7) };
    
    html! {
        <div class="calendar-app">
            <div class="calendar">
                <h2 class="calendar-heading">{ "Agenda" }</h2>
                <div class="navigate-date">
                    <h2 class="month"> { months_of_year[*current_month as usize - 1] } </h2>
                    <h2 class="year"> { *current_year } </h2>
                    <div class="calendar-buttons">
                        <button onclick={prev_month}>{ "<" }</button>
                        <button onclick={next_month}>{ ">" }</button>
                        <button onclick={toggle_task_form}>{ "+" }</button>
                    </div>
                </div>
                <div class="weekdays">
                    {
                        for days_of_week.iter().map(|&day| html! {
                            <span class="weekday">{ day }</span>
                        }) 
                    }
                </div>
                <div class="days">
                    
                    { for (0..first_weekday).map(|index| html! {
                        <span key={format!("empty-{}", index)} class="empty-day"></span>
                    }) }
                    
                    { for (1..=days_in_month).map(|day| {
                        let on_day_click = on_day_click.clone();
                        let is_today = day == current_date.day() && 
                                      *current_month == current_date.month() && 
                                      *current_year == current_date.year();
                        let is_selected = day == *selected_day;
                        
                        let class = match (is_today, is_selected) {
                            (true, true) => "unique-day current-day selected-day",
                            (true, false) => "unique-day current-day",
                            (false, true) => "unique-day selected-day",
                            (false, false) => "unique-day",
                        };
                        
                        html! {
                            <span 
                                class={class}
                                onclick={Callback::from(move |_: MouseEvent| {
                                    on_day_click.emit(day);
                                })}
                            >
                                { day }
                            </span>
                        }
                    }) }
                    { 
                        for (0..remaining_cells).map(|index| html! {
                            <span key={format!("empty-end-{}", index)} class="empty-day"></span>
                        })
                    }
                </div>
            </div>
            
            <div class="sidebar">
                <div class="sidebar-header">
                    <h3>{ "Tarefas" }</h3>
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
                selected_date={NaiveDate::from_ymd_opt(*current_year, *current_month, *selected_day).unwrap_or_else(|| Local::now().date_naive())}
            />
        </div>
    }
}