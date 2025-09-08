use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect, use_state, Callback, Html, MouseEvent};
use chrono::{Local, NaiveDate, Datelike};

use crate::components::{task_card::TaskCard, task_form::TaskForm};
use crate::components::{reminder_form::ReminderForm};
use crate::types::{TaskDuration, Task};

#[function_component(CalendarApp)]
pub fn calendar_app() -> Html {
    let show_task_form = use_state(|| false);
    let tasks = use_state(|| Vec::<Task>::new());
    let first_render = use_state(|| true);


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

    let show_reminder_form = use_state(|| false);

    let toggle_reminder_form = {
        let show_reminder_form = show_reminder_form.clone();
        Callback::from(move |_: MouseEvent| {
            show_reminder_form.set(!*show_reminder_form);
        })
    };

    let close_reminder_form = {
        let show_reminder_form = show_reminder_form.clone();
        Callback::from(move |_: ()| {
            show_reminder_form.set(false);
        })
};
    let on_task_delete = {
        let tasks = tasks.clone();
        Callback::from(move |task_id: u32| {
            let tasks = tasks.clone();
            spawn_local(async move {
                match crate::services::tasks::delete_task(task_id).await {
                    Ok(_) => {
                        let updated_tasks: Vec<Task> = (*tasks)
                            .iter()
                            .cloned()
                            .filter(|task| task.id != task_id)
                            .collect();
                        tasks.set(updated_tasks);
                    }
                    Err(error) => {
                        web_sys::console::log_1(&format!("Failed to delete task: {}", error).into());
                    }
                }
            });
        })
    };

    let on_task_created = {
        let tasks = tasks.clone();
        Callback::from(move |new_task: Task| {
            let mut current_tasks = (*tasks).clone();
            current_tasks.push(new_task);
            tasks.set(current_tasks);
        })
    };

    {
        let tasks = tasks.clone();
        use_effect(move || {
            if *first_render {
                first_render.set(false);
            } else {
                return;
            }
            spawn_local(async move {
                match crate::services::tasks::get_all_tasks().await {
                    Ok(fetched_tasks) => {
                        tasks.set(fetched_tasks);
                    }
                    Err(error) => {
                        web_sys::console::log_1(&format!("Failed to fetch tasks: {}", error).into());
                    }
                }
            });
        });
    }

    let current_date = Local::now().date_naive();

    let current_month = use_state(|| current_date.month());
    let current_year = use_state(|| current_date.year());
    let selected_day = use_state(|| current_date.day());

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
                        <button onclick={toggle_reminder_form}>{ "Lembrete" }</button>
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
                    { 
                        for tasks.iter().enumerate().map(|(index, task)| {
                            let duration = TaskDuration::from_value(&task.task_type).unwrap_or_default();
                            let date_formatted = task.begin_date.format("%B %d, %Y").to_string();
                            let time_formatted = task.begin_date.format("%H:%M").to_string();
                            
                            html! {
                                <TaskCard 
                                    key={format!("task-{}-{}", index, task.title)}
                                    id={task.id}
                                    title={task.title.clone()}
                                    category={task.category.clone()}
                                    description={task.description.clone()}
                                    on_task_delete={on_task_delete.clone()}
                                    date={date_formatted}
                                    time={time_formatted}
                                    duration={duration}
                                />
                            }
                        })
                    }
                </div>
            </div>
            
            <TaskForm 
                visible={*show_task_form} 
                on_close={close_task_form}
                on_task_created={on_task_created}
                selected_date={NaiveDate::from_ymd_opt(*current_year, *current_month, *selected_day).unwrap_or_else(|| Local::now().date_naive())}
            />

            <ReminderForm 
                visible={*show_reminder_form} 
                on_close={close_reminder_form.clone()}
            />
        </div>
    }
}