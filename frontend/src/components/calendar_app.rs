use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect, use_state, Callback, Html, MouseEvent, Properties};
use chrono::{Local, NaiveDate, Datelike};
use chrono::TimeZone;
use crate::components::{task_card::TaskCard, task_form::TaskForm};
use crate::components::{reminder_form::ReminderForm, reminder_card::ReminderCard};
use crate::types::{TaskDuration, Task};
use crate::services::tasks::{TaskDto, TaskUpdateDto};
use crate::types::reminder::Reminder;
use web_sys::HtmlAudioElement;

#[derive(Clone, PartialEq)]
pub enum ViewType {
    Tasks,
    Reminders,
    Goals,
}

// Mock Goal struct for now
#[derive(Clone, Debug)]
pub struct Goal {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub due_date: String,
}

#[derive(Properties, PartialEq)]
pub struct CalendarAppProps {
    pub visible: bool,
    pub on_close: Callback<()>,
}

#[function_component(CalendarApp)]
pub fn calendar_app(props: &CalendarAppProps) -> Html {
    let show_task_form = use_state(|| false);
    let tasks = use_state(|| Vec::<Task>::new());
    let reminders = use_state(|| Vec::<Reminder>::new());
    let goals = use_state(|| Vec::<Goal>::new());
    let first_render = use_state(|| true);
    let current_view = use_state(|| ViewType::Tasks);

    // View switching callbacks
    let switch_to_tasks = {
        let current_view = current_view.clone();
        Callback::from(move |_: MouseEvent| {
            current_view.set(ViewType::Tasks);
        })
    };

    let switch_to_reminders = {
        let current_view = current_view.clone();
        Callback::from(move |_: MouseEvent| {
            current_view.set(ViewType::Reminders);
        })
    };

    let switch_to_goals = {
        let current_view = current_view.clone();
        Callback::from(move |_: MouseEvent| {
            current_view.set(ViewType::Goals);
        })
    };


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

    let on_task_update = {
        let tasks = tasks.clone();
        Callback::from(move |(task_id, new_title, new_description): (u32, String, String)| {
            let tasks = tasks.clone();
            let tasks_for_find = tasks.clone();
            spawn_local(async move {
                // First, find the current task to get all its fields
                if let Some(current_task) = (*tasks_for_find).iter().find(|t| t.id == task_id) {
                    // Create TaskDto with all required fields, updating only title and description
                    let task_dto = TaskUpdateDto {
                        title: new_title.clone(),
                        description: new_description.clone(),
                        status: current_task.status.clone(),
                    };
                    
                    match crate::services::tasks::update_task_with_dto(task_id, task_dto).await {
                        Ok(_) => {
                            let updated_tasks: Vec<Task> = (*tasks)
                                .iter()
                                .map(|task| {
                                    if task.id == task_id {
                                        let mut updated_task = task.clone();
                                        updated_task.title = new_title.clone();
                                        updated_task.description = new_description.clone();
                                        updated_task
                                    } else {
                                        task.clone()
                                    }
                                })
                                .collect();
                            tasks.set(updated_tasks);
                        }
                        Err(error) => {
                            web_sys::console::log_1(&format!("Failed to update task: {}", error).into());
                        }
                    }
                } else {
                    web_sys::console::log_1(&"Task not found for update".into());
                }
            });
        })
    };

    let on_reminder_update = {
        let reminders = reminders.clone();
        Callback::from(move |(reminder_id, new_name, new_category, new_date_end): (i32, String, String, String)| {
            let reminders = reminders.clone();
            let new_date = chrono::NaiveDate::parse_from_str(&new_date_end, "%Y-%m-%d")
                .map(|d| chrono::NaiveDateTime::new(d, chrono::NaiveTime::from_hms_opt(0,0,0).unwrap()))
                .ok()
                .map(|naive| chrono::Utc.from_utc_datetime(&naive));
            if let Some(date_end) = new_date {
                let dto = crate::services::reminder_service::ReminderUpdateDto {
                    name: new_name.clone(),
                    category: new_category.clone(),
                    date_end,
                };
                wasm_bindgen_futures::spawn_local({
                    let reminders = reminders.clone();
                    async move {
                        let _ = crate::services::reminder_service::update_reminder(reminder_id as u32, dto).await;
                        let updated_reminders: Vec<Reminder> = (*reminders)
                            .iter()
                            .cloned()
                            .map(|mut reminder| {
                                if reminder.id == reminder_id {
                                    reminder.name = new_name.clone();
                                    reminder.category = new_category.clone();
                                    reminder.date_end = date_end;
                                }
                                reminder
                            })
                            .collect();
                        reminders.set(updated_reminders);
                    }
                });
            }
        })
    };
    let on_reminder_delete = {
        let reminders = reminders.clone();
        Callback::from(move |reminder_id: i32| {
            let reminders = reminders.clone();
            spawn_local(async move {
                match crate::services::reminder_service::delete_reminder(reminder_id as u32).await {
                    Ok(_) => {
                        let updated_reminders: Vec<Reminder> = (*reminders)
                            .iter()
                            .cloned()
                            .filter(|reminder| reminder.id != reminder_id)
                            .collect();
                        reminders.set(updated_reminders);
                    }
                    Err(error) => {
                        web_sys::console::log_1(&format!("Failed to delete reminder: {}", error).into());
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

    let on_reminder_created = {
        let reminders = reminders.clone();
        Callback::from(move |new_reminder: Reminder| {
            let mut current_reminders = (*reminders).clone();
            current_reminders.push(new_reminder);
            reminders.set(current_reminders);
        })
    };

    let on_status_update = {
        let tasks = tasks.clone();
        Callback::from(move |(task_id, new_status): (u32, String)| {
            let tasks = tasks.clone();
            spawn_local(async move {
                if let Some(task_index) = (*tasks).iter().position(|t| t.id == task_id) {
                    let mut task = (*tasks)[task_index].clone();
                    task.status = new_status.clone();
                    
                    // Create TaskDto for update
                    let task_dto = TaskUpdateDto {

                        title: task.title.clone(),
                        description: task.description.clone(),
                        status: new_status,
                    };

                    match crate::services::tasks::update_task_with_dto(task_id, task_dto).await {
                        Ok(_) => {
                            // Update local state
                            let mut updated_tasks = (*tasks).clone();
                            updated_tasks[task_index] = task;
                            tasks.set(updated_tasks);
                        }
                        Err(error) => {
                            web_sys::console::log_1(&format!("Failed to update task status: {}", error).into());
                        }
                    }
                } else {
                    web_sys::console::log_1(&"Task not found for status update".into());
                }
            });
        })
    };

    {
        let tasks = tasks.clone();
        let reminders = reminders.clone();
        let goals = goals.clone();
        use_effect(move || {
            if *first_render {
                first_render.set(false);
            } else {
                return;
            }
            
            let audio_element = HtmlAudioElement::new_with_src("/Windows_XP_Startup.wav").unwrap();
            let _ = audio_element.play().unwrap();
            let mock_goals = vec![
                Goal {
                    id: 1,
                    title: "Aprender Rust".to_string(),
                    description: "Concluir curso de Rust avançado".to_string(),
                    status: "Em Progresso".to_string(),
                    due_date: "Dezembro 2025".to_string(),
                },
                Goal {
                    id: 2,
                    title: "Projeto Agenda".to_string(),
                    description: "Finalizar aplicação de agenda".to_string(),
                    status: "Em Progresso".to_string(),
                    due_date: "Novembro 2025".to_string(),
                },
                Goal {
                    id: 3,
                    title: "Exercícios".to_string(),
                    description: "Fazer exercícios 3x por semana".to_string(),
                    status: "Pendente".to_string(),
                    due_date: "Contínuo".to_string(),
                },
            ];
            goals.set(mock_goals);
            
            spawn_local(async move {
                match crate::services::tasks::get_all_tasks().await {
                    Ok(fetched_tasks) => {
                        tasks.set(fetched_tasks);
                    }
                    Err(error) => {
                        web_sys::console::log_1(&format!("Failed to fetch tasks: {}", error).into());
                    }
                }
                match crate::services::reminder_service::get_all_reminders().await {
                    Ok(fetched_reminders) => {
                        reminders.set(fetched_reminders);
                    }
                    Err(error) => {
                        web_sys::console::log_1(&format!("Failed to fetch reminders: {}", error).into());
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

    // Function to count tasks for a specific date
    let count_tasks_for_date = |day: u32| -> usize {
        let target_date = NaiveDate::from_ymd_opt(*current_year, *current_month, day)
            .unwrap_or_else(|| Local::now().date_naive());
        
        tasks.iter().filter(|task| {
            task.begin_date.date_naive() == target_date
        }).count()
    };
    
    html! {
        if !props.visible {
            <div></div>
        } else {
        <div class="calendar-app">
            <div class="calendar-header">
                <h1 class="calendar-title">{ "Agenda - Windows 98" }</h1>
                <div class="calendar-header-controls">
                    <button class="control-button minimize-btn" type="button" onclick={
                        let on_close = props.on_close.clone();
                        Callback::from(move |_: MouseEvent| {
                            on_close.emit(());
                        })
                    }>
                    </button>
                    <button class="control-button maximize-btn" type="button">
                    </button>
                    <button class="control-button close-btn" type="button" onclick={
                        let on_close = props.on_close.clone();
                        Callback::from(move |_: MouseEvent| {
                            on_close.emit(());
                        })
                    }>
                    </button>
                </div>
            </div>
            <div class="calendar-content">
                <div class="calendar">
                    <h2 class="calendar-heading">{ "Agenda" }</h2>
                <div class="navigate-date">
                    <h2 class="month"> { months_of_year[*current_month as usize - 1] } </h2>
                    <h2 class="year"> { *current_year } </h2>
                    <div class="calendar-buttons">
                        <button onclick={prev_month}>{ "<" }</button>
                        <button onclick={next_month}>{ ">" }</button>
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
                        let task_count = count_tasks_for_date(day);
                        
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
                                <span class="day-number">{ day }</span>
                                { if task_count > 0 {
                                    html! { <span class="task-count">{ task_count }</span> }
                                } else {
                                    html! {}
                                }}
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
                    <div class="view-buttons">
                        <button 
                            class={if *current_view == ViewType::Tasks { "view-btn active" } else { "view-btn" }}
                            onclick={switch_to_tasks}
                        >
                            { "Tarefas" }
                        </button>
                        <button 
                            class={if *current_view == ViewType::Reminders { "view-btn active" } else { "view-btn" }}
                            onclick={switch_to_reminders}
                        >
                            { "Lembretes" }
                        </button>
                        <button 
                            class={if *current_view == ViewType::Goals { "view-btn active" } else { "view-btn" }}
                            onclick={switch_to_goals}
                        >
                            { "Metas" }
                        </button>
                    </div>
                    <div class="header-row">
                        <h3>
                            { match &*current_view {
                                ViewType::Tasks => "Tarefas",
                                ViewType::Reminders => "Lembretes", 
                                ViewType::Goals => "Metas",
                            }}
                        </h3>
                        <div class="action-buttons">
                            { match &*current_view {
                                ViewType::Tasks => html! {
                                    <button class="add-btn" onclick={toggle_task_form}>{ "Nova Tarefa" }</button>
                                },
                                ViewType::Reminders => html! {
                                    <button class="add-btn" onclick={toggle_reminder_form}>{ "Novo Lembrete" }</button>
                                },
                                ViewType::Goals => html! {
                                    <button class="add-btn" onclick={Callback::from(|_: MouseEvent| {
                                        web_sys::console::log_1(&"Add Goal clicked".into());
                                    })}>{ "Nova Meta" }</button>
                                },
                            }}
                        </div>
                    </div>
                </div>
                <div class="content-list">
                    { match &*current_view {
                        ViewType::Tasks => {
                            // Filter tasks by selected date
                            let selected_date = NaiveDate::from_ymd_opt(*current_year, *current_month, *selected_day)
                                .unwrap_or_else(|| Local::now().date_naive());
                            
                            let filtered_tasks: Vec<&Task> = tasks.iter()
                                .filter(|task| task.begin_date.date_naive() == selected_date)
                                .collect();
                            
                            let task_cards: Vec<Html> = filtered_tasks.iter().enumerate().map(|(index, task)| {
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
                                        on_task_update={Some(on_task_update.clone())}
                                        on_status_update={Some(on_status_update.clone())}
                                        status={task.status.clone()}
                                        date={date_formatted}
                                        time={time_formatted}
                                        duration={duration}
                                    />
                                }
                            }).collect();
                            
                            if task_cards.is_empty() {
                                html! { <></> }
                            } else {
                                html! { <>{task_cards}</> }
                            }
                        },
                        ViewType::Reminders => {
                            // Calcular início (domingo) e fim (sábado) da semana do dia selecionado
                            let selected_date = NaiveDate::from_ymd_opt(*current_year, *current_month, *selected_day)
                                .unwrap_or_else(|| Local::now().date_naive());
                            let weekday = selected_date.weekday().number_from_sunday(); // 1 = domingo, 7 = sábado
                            let start_of_week = selected_date - chrono::Duration::days((weekday - 1) as i64);
                            let end_of_week = selected_date + chrono::Duration::days((7 - weekday) as i64);

                            // Converter para DateTime<Utc> para comparar com reminder.date_end
                            use chrono::{NaiveDateTime, Utc, TimeZone};
                            let start_of_week_dt = Utc.from_utc_datetime(&start_of_week.and_hms_opt(0, 0, 0).unwrap());
                            let end_of_week_dt = Utc.from_utc_datetime(&end_of_week.and_hms_opt(23, 59, 59).unwrap());

                            // Filtrar lembretes da semana
                            let weekly_reminders: Vec<&Reminder> = reminders.iter()
                                .filter(|reminder| {
                                    reminder.date_end >= start_of_week_dt && reminder.date_end <= end_of_week_dt
                                })
                                .collect();

                            let reminder_cards: Vec<Html> = weekly_reminders.iter().enumerate().map(|(index, reminder)| {
                                html! {
                                    <ReminderCard 
                                        key={format!("reminder-{}-{}", index, reminder.name)}
                                        id={reminder.id}
                                        name={reminder.name.clone()}
                                        category={reminder.category.clone()}
                                        date_end={reminder.date_end}
                                        on_reminder_delete={on_reminder_delete.clone()}
                                        on_reminder_update={Some(on_reminder_update.clone())}
                                    />
                                }
                            }).collect();

                            html! { <>{reminder_cards}</> }
                        },
                        ViewType::Goals => {
                            let goal_cards: Vec<Html> = goals.iter().enumerate().map(|(index, goal)| {
                                html! {
                                    <div key={format!("goal-{}-{}", index, goal.title)} class="goal-card">
                                        <h4>{ &goal.title }</h4>
                                        <p>{ &goal.description }</p>
                                        <div class="goal-meta">
                                            <span class={format!("status {}", goal.status.to_lowercase().replace(" ", "-"))}>
                                                { &goal.status }
                                            </span>
                                            <span class="due-date">{ &goal.due_date }</span>
                                        </div>
                                    </div>
                                }
                            }).collect();
                            
                            html! { <>{goal_cards}</> }
                        }
                    }}
                </div>
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
                on_reminder_created={on_reminder_created}
            />
        </div>
    }
}
}