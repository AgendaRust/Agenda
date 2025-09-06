use yew::{function_component, html, use_state, Callback, Event, Html, InputEvent, MouseEvent, Properties, TargetCast};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use chrono::NaiveDate;
use crate::types::TaskDuration;
use crate::services::tasks::{TaskDto, create_task};
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
pub struct TaskFormProps {
    pub visible: bool,
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
    pub selected_date: NaiveDate,
}

#[function_component(TaskForm)]
pub fn task_form(props: &TaskFormProps) -> Html {

    let task_title = use_state(|| String::new());
    let task_category = use_state(|| String::new());
    let task_description = use_state(|| String::new());
    let task_hour = use_state(|| 9u32);
    let task_minute = use_state(|| 0u32);
    let task_type = use_state(|| TaskDuration::default());
    let form_status = use_state(|| String::new()); // "success", "error", or ""

    let begin_date = format!("{}T{:02}:{:02}", 
        props.selected_date.format("%Y-%m-%d"), 
        *task_hour, 
        *task_minute
    );

    let on_hour_change = {
        let task_hour = task_hour.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            
            if value.is_empty() {
                task_hour.set(0);
                return;
            }
            
            if let Ok(num) = value.parse::<u32>() {
                if num <= 23 {
                    task_hour.set(num);
                } else {
                    task_hour.set(23);
                    input.set_value("23");
                }
            } else {
                input.set_value(&task_hour.to_string());
            }
        })
    };

    let on_minute_change = {
        let task_minute = task_minute.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            
            if value.is_empty() {
                task_minute.set(0);
                return;
            }
            
            if let Ok(num) = value.parse::<u32>() {
                if num <= 59 {
                    task_minute.set(num);
                } else {
                    task_minute.set(59);
                    input.set_value("59");
                }
            } else {
                input.set_value(&task_minute.to_string());
            }
        })
    };

    let on_create = {
        let task_title = task_title.clone();
        let task_category = task_category.clone();
        let task_description = task_description.clone();
        let task_type = task_type.clone();
        let form_status = form_status.clone();
        let on_close = props.on_close.clone();
        let begin_date = begin_date.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let task_title = task_title.clone();
            let task_category = task_category.clone();
            let task_description = task_description.clone();
            let task_type = task_type.clone();
            let form_status = form_status.clone();
            let on_close = on_close.clone();
            let begin_date = begin_date.clone();
            
            spawn_local(async move {
                let begin_date_parsed = chrono::NaiveDateTime::parse_from_str(&begin_date, "%Y-%m-%dT%H:%M")
                                    .ok()
                                    .map(|naive| chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc))
                                    .unwrap_or_else(|| chrono::Utc::now());

                let task_info = TaskDto {
                    title: (*task_title).clone(),
                    category: (*task_category).clone(),
                    description: (*task_description).clone(),
                    begin_date: begin_date_parsed,
                    task_type: task_type.value().to_string(),
                };
                
                let result = create_task(&task_info).await;
                match result {
                    crate::services::tasks::TaskResult::Success(task) => {
                        web_sys::console::log_1(&format!("Task created successfully: {:?}", task).into());
                        form_status.set("success".to_string());
                        
                        // Clear form and close after success animation
                        let task_title = task_title.clone();
                        let task_category = task_category.clone();
                        let task_description = task_description.clone();
                        let task_type = task_type.clone();
                        let form_status = form_status.clone();
                        let on_close = on_close.clone();
                        
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(1500).await;
                            task_title.set(String::new());
                            task_category.set(String::new());
                            task_description.set(String::new());
                            task_type.set(TaskDuration::default());
                            form_status.set(String::new());
                            
                            if let Some(callback) = &on_close {
                                callback.emit(());
                            }
                        });
                    },
                    crate::services::tasks::TaskResult::InvalidFields => {
                        web_sys::console::log_1(&"Failed to create task: Invalid fields".into());
                        form_status.set("error".to_string());
                        
                        // Reset error status after animation
                        let form_status = form_status.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(3000).await;
                            form_status.set(String::new());
                        });
                    },
                    crate::services::tasks::TaskResult::NetworkError(err) => {
                        web_sys::console::log_1(&format!("Network error while creating task: {}", err).into());
                        form_status.set("error".to_string());
                        
                        // Reset error status after animation
                        let form_status = form_status.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(3000).await;
                            form_status.set(String::new());
                        });
                    },
                }
            });
        })
    };


    let on_title_change = {
        let task_title = task_title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            task_title.set(input.value());
        })
    };

    let on_category_change = {
        let task_category = task_category.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            task_category.set(input.value());
        })
    };

    let on_description_change = {
        let task_description = task_description.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: HtmlTextAreaElement = e.target_unchecked_into();
            task_description.set(textarea.value());
        })
    };

    let on_type_change = {
        let task_type = task_type.clone();
        Callback::from(move |e: Event| {
            let select: HtmlInputElement = e.target_unchecked_into();
            //test with different dataset
            let duration = TaskDuration::from_value(&select.value()).unwrap_or_default();

            task_type.set(duration);
        })
    };

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(callback) = &on_close {
                callback.emit(());
            }
        })
    };

    html! {
        if props.visible {
            <div class="task-popup">
                <div class={format!("task-form {}", (*form_status).clone())}>
                    // Windows 98 Title Bar Header
                    <div class="task-form-header">
                        <div class="title-text">{"Criar tarefa"}</div>
                        <div class="window-controls">
                            <div class="control-button minimize"></div>
                            <div class="control-button maximize"></div>
                            <div class="control-button close" onclick={on_close.clone()}></div>
                        </div>
                    </div>

                    // Task Form Content
                    <div class="task-form-content">
                        // Status message
                        if !form_status.is_empty() {
                            <div class={format!("status-message {}", (*form_status).clone())}>
                                if *form_status == "success" {
                                    { "✓ Task created successfully!" }
                                } else if *form_status == "error" {
                                    { "✗ Failed to create task. Please try again." }
                                }
                            </div>
                        }

                        // Time input - left column
                        <div>
                            <label>{ "Hora:" }</label>
                            <div class="time-input">
                                <div class="event-popup-time">
                                    <input 
                                        type="number" 
                                        name="hours" 
                                        min="0" 
                                        max="23" 
                                        class="hour-input" 
                                        placeholder="HH" 
                                        value={task_hour.to_string()}
                                        oninput={on_hour_change}
                                    />
                                    { ":" }
                                    <input 
                                        type="number" 
                                        name="minutes" 
                                        min="0" 
                                        max="59" 
                                        class="minute-input" 
                                        placeholder="MM" 
                                        value={task_minute.to_string()}
                                        oninput={on_minute_change}
                                    />
                                </div>
                            </div>
                            
                            // Debug display (you can remove this later)
                            <div style="font-size: 8px; color: #666; margin-top: 4px;">
                                { format!("Data/Hora: {}", begin_date) }
                            </div>
                        </div>

                        // Task title - right column
                        <div>
                            <label for="title">{ "Nova task:" }</label>
                            <input 
                                type="text" 
                                id="title" 
                                name="title" 
                                minlength="3" 
                                required=true 
                                placeholder="Digite o título da task"
                                value={(*task_title).clone()}
                                oninput={on_title_change}
                            />
                        </div>

                        // Category - left column
                        <div>
                            <label for="category">{ "Categoria:" }</label>
                            <input 
                                type="text" 
                                id="category" 
                                name="category" 
                                minlength="5" 
                                required=true 
                                placeholder="Digite a categoria"
                                value={(*task_category).clone()}
                                oninput={on_category_change}
                            />
                        </div>

                        // Type - right column
                        <div>
                            <label for="type">{ "Tipo:" }</label>
                            <select 
                                id="type" 
                                name="type" 
                                required=true
                                onchange={on_type_change}
                            >
                                {
                                    TaskDuration::all().iter().map(|duration| {
                                        let is_selected = *task_type == *duration;
                                        html! {
                                            <option 
                                                value={duration.value()} 
                                                selected={is_selected}
                                            >
                                                { duration.display_name() }
                                            </option>
                                        }
                                    }).collect::<Html>()
                                }
                            </select>
                        </div>

                        // Description - full width
                        <div class="full-width">
                            <label for="description">{ "Descrição:" }</label>
                            <textarea 
                                id="description" 
                                name="description" 
                                required=true 
                                placeholder="Digite a descrição"
                                rows="3"
                                value={(*task_description).clone()}
                                oninput={on_description_change}
                            ></textarea>
                        </div>

                        // Buttons - full width
                        <div class="button-container">
                            <button type="submit" onclick={on_create}>{"Criar Task"}</button>
                            <button type="button" onclick={on_close}>{"Cancelar"}</button>
                        </div>

                        // Hidden input for begin_date (combined date + time)
                        <input 
                            type="hidden" 
                            name="begin_date" 
                            value={begin_date.clone()}
                        />
                    </div>
                </div>
            </div>
        }
    }
}