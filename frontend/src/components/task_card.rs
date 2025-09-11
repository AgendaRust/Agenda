use yew::{function_component, html, Html, Properties, use_state, Callback, MouseEvent, InputEvent, TargetCast};
use web_sys::HtmlInputElement;
use crate::types::TaskDuration;

fn format_time_display(time: &str, duration: &TaskDuration) -> String {
    match duration {
        TaskDuration::Manha => "Manhã".to_string(),
        TaskDuration::Tarde => "Tarde".to_string(),
        TaskDuration::Noite => "Noite".to_string(),
        TaskDuration::MeiaHora | TaskDuration::UmaHora => {
            // Parse the time (format: "HH:MM")
            if let Some((hour_str, minute_str)) = time.split_once(':') {
                if let (Ok(hour), Ok(minute)) = (hour_str.parse::<u32>(), minute_str.parse::<u32>()) {
                    let start_time = format!("{:02}:{:02}", hour, minute);
                    
                    let duration_minutes = match duration {
                        TaskDuration::MeiaHora => 30,
                        TaskDuration::UmaHora => 60,
                        _ => 0,
                    };
                    
                    let total_minutes = hour * 60 + minute + duration_minutes;
                    let end_hour = (total_minutes / 60) % 24; // Handle day overflow
                    let end_minute = total_minutes % 60;
                    let end_time = format!("{:02}:{:02}", end_hour, end_minute);
                    
                    format!("{} - {}", start_time, end_time)
                } else {
                    format!("Time: {}", time)
                }
            } else {
                format!("Time: {}", time)
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TaskCardProps {
    pub id: u32,
    pub title: String,
    pub category: String,
    pub description: String,
    pub date: String,
    pub time: String,
    pub duration: TaskDuration,
    pub status: String,
    pub on_task_delete: Callback<u32>,
    pub on_task_update: Option<Callback<(u32, String, String)>>, // (id, title, description)
    pub on_status_update: Option<Callback<(u32, String)>>, // (id, new_status)
}

#[function_component(TaskCard)]
pub fn task_card(props: &TaskCardProps) -> Html {
    let show_info = use_state(|| false);
    let is_editing = use_state(|| false);
    let edit_title = use_state(|| props.title.clone());
    let edit_description = use_state(|| props.description.clone());

    let toggle_info = {
        let show_info = show_info.clone();
        let is_editing = is_editing.clone();
        Callback::from(move |_: MouseEvent| {
            if !*is_editing {
                show_info.set(!*show_info);
            }
        })
    };

    let on_edit_click = {
        let is_editing = is_editing.clone();
        let edit_title = edit_title.clone();
        let edit_description = edit_description.clone();
        let props_title = props.title.clone();
        let props_description = props.description.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            is_editing.set(true);
            edit_title.set(props_title.clone());
            edit_description.set(props_description.clone());
        })
    };

    let on_save_click = {
        let is_editing = is_editing.clone();
        let edit_title = edit_title.clone();
        let edit_description = edit_description.clone();
        let on_task_update = props.on_task_update.clone();
        let task_id = props.id;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            
            if let Some(callback) = &on_task_update {
                callback.emit((task_id, (*edit_title).clone(), (*edit_description).clone()));
            }
            is_editing.set(false);
        })
    };

    let on_cancel_click = {
        let is_editing = is_editing.clone();
        let edit_title = edit_title.clone();
        let edit_description = edit_description.clone();
        let props_title = props.title.clone();
        let props_description = props.description.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            is_editing.set(false);
            edit_title.set(props_title.clone());
            edit_description.set(props_description.clone());
        })
    };

    let on_title_input = {
        let edit_title = edit_title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            edit_title.set(input.value());
        })
    };

    let on_description_input = {
        let edit_description = edit_description.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            edit_description.set(input.value());
        })
    };

    let on_status_toggle = {
        let on_status_update = props.on_status_update.clone();
        let task_id = props.id;
        let current_status = props.status.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            
            let new_status = if current_status.to_lowercase() == "pendente" {
                "Concluída".to_string()
            } else {
                "Pendente".to_string()
            };
            
            if let Some(callback) = &on_status_update {
                callback.emit((task_id, new_status));
            }
        })
    };

    let task_id = props.id;
    let on_delete_click = {
        let on_task_delete = props.on_task_delete.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            web_sys::console::log_1(&"Delete button clicked".into());
            on_task_delete.emit(task_id);
        })
    };

    html! {
        <div class="task-card" onclick={toggle_info}>
            <div class="task-header">
                if *is_editing {
                    <input 
                        class="task-title-input"
                        type="text"
                        value={(*edit_title).clone()}
                        oninput={on_title_input}
                        onclick={Callback::from(|e: MouseEvent| {
                            e.prevent_default();
                            e.stop_propagation();
                        })}
                    />
                } else {
                    <h3 class="task-title">{ &props.title }</h3>
                }
                if *show_info {
                    <div class="task-actions">
                        if *is_editing {
                            <button class="save-button" onclick={on_save_click}>{ "Salvar" }</button>
                            <button class="cancel-button" onclick={on_cancel_click}>{ "Cancelar" }</button>
                        } else {
                            <button class="edit-button" onclick={on_edit_click}>{ "Editar" }</button>
                            <button class="delete-button" onclick={on_delete_click}>{ "Excluir" }</button>
                        }
                    </div>
                }
            </div>
            <div class="task-body">
                if *show_info {
                    if *is_editing {
                        <textarea 
                            class="task-description-input"
                            value={(*edit_description).clone()}
                            oninput={on_description_input}
                            onclick={Callback::from(|e: MouseEvent| {
                                e.prevent_default();
                                e.stop_propagation();
                            })}
                        />
                    } else {
                        <p class="task-description">{ &props.description }</p>
                    }
                    <div class="task-status">
                        <span class="status-label">{ "Status: " }</span>
                        <span class="status-value">{ &props.status }</span>
                    </div>
                }
                <div class="task-datetime">
                    <span class="task-date">{ format!("Due Date: {}", &props.date) }</span>
                    <span class="task-time">{ format_time_display(&props.time, &props.duration) }</span>
                </div>
                if *show_info {
                    <div class="task-status-actions">
                        <button 
                            class={if props.status.to_lowercase() == "pendente" { "complete-button" } else { "incomplete-button" }}
                            onclick={on_status_toggle}
                        >
                            { if props.status.to_lowercase() == "pendente" { "Completar" } else { "Descompletar" } }
                        </button>
                    </div>
                }
            </div>
        </div>
    }
}