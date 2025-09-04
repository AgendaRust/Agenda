use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

use crate::components::ThemeToggle;
use crate::hooks::use_theme;
use crate::services::tasks::{create_task, TaskDto, TaskResult};

#[function_component(Home)]
pub fn home_component() -> Html {
    let counter = use_state(|| 0);
    let (dark_mode, toggle_theme) = use_theme();

    // Task form state variables
    let task_title = use_state(|| String::new());
    let task_category = use_state(|| String::new());
    let task_description = use_state(|| String::new());
    let task_begin_date = use_state(|| String::new());
    let task_type = use_state(|| "MeiaHora".to_string());

    let add_one = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter + 1);
        })
    };

    let minus_one = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(*counter - 1);
        })
    };

    let reset_counter = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(0);
        })
    };

    let theme_class = if dark_mode {
        "dark-theme"
    } else {
        "light-theme"
    };

    // Task form submit handler
    let on_task_submit = {
        let task_title = task_title.clone();
        let task_category = task_category.clone();
        let task_description = task_description.clone();
        let task_begin_date = task_begin_date.clone();
        let task_type = task_type.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let title = (*task_title).clone();
            let category = (*task_category).clone();
            let description = (*task_description).clone();
            let begin_date = (*task_begin_date).clone();
            let task_type_val = (*task_type).clone();
            
            // Send request to backend
            spawn_local(async move {
                // Validate required fields
                if title.is_empty() || category.len() < 5 || begin_date.is_empty() {
                    web_sys::console::log_1(&"Please fill all required fields".into());
                    return;
                }

                // Parse the datetime-local input (format: YYYY-MM-DDTHH:MM)
                let parsed_date = match chrono::NaiveDateTime::parse_from_str(&begin_date, "%Y-%m-%dT%H:%M") {
                    Ok(naive_dt) => {
                        // Convert to UTC DateTime
                        chrono::DateTime::from_naive_utc_and_offset(naive_dt, chrono::Utc)
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Date parsing error: {}", e).into());
                        return;
                    }
                };

                let task_info = TaskDto {
                    title,
                    category,
                    description,
                    begin_date: parsed_date,
                    task_type: task_type_val,
                };
                
                // Debug: Log the task info being sent
                web_sys::console::log_1(&format!("Sending task: {:?}", task_info).into());
                
                match create_task(&task_info).await {
                    TaskResult::Success(task) => {
                        web_sys::console::log_1(&format!("Task created: {:?}", task).into());
                        // Handle successful task creation
                    }
                    TaskResult::InvalidFields => {
                        // Handle invalid fields
                    }
                    TaskResult::NetworkError(err) => {
                        web_sys::console::log_1(&format!("Network error: {}", err).into());
                        // Handle network error
                    }
                }
            });
            
            // Clear form after submission
            task_title.set(String::new());
            task_category.set(String::new());
            task_description.set(String::new());
            task_begin_date.set(String::new());
            task_type.set("MeiaHora".to_string());
        })
    };

    html! {
        <>
            <ThemeToggle dark_mode={dark_mode} on_toggle={toggle_theme} />
            <div class={format!("container {}", theme_class)}>
                <h1> { "Bem vindo ao nosso projeto de agenda com rust!" } </h1>
                <div class="button-container">
                    <button onclick={add_one}>{ "+1" }</button>
                    <button onclick={reset_counter}>{ "Reset" }</button>
                    <button onclick={minus_one}>{ "-1" }</button>
                </div>
                <p class="counter">{ *counter }</p>
            </div>
            <div class={format!("form-container {}", theme_class)}>
                <form class="task-form" onsubmit={on_task_submit}>
                    <label for="title">{ "Nova task:" }</label>
                    <input 
                        type="text" 
                        id="title" 
                        name="title" 
                        minlength="3" 
                        required=true 
                        value={(*task_title).clone()}
                        oninput={Callback::from({
                            let task_title = task_title.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                task_title.set(input.value());
                            }
                        })}
                    />
                    <label for="category">{ "Categoria:" }</label>
                    <input 
                        type="text" 
                        id="category" 
                        name="category" 
                        minlength="5" 
                        required=true 
                        value={(*task_category).clone()}
                        oninput={Callback::from({
                            let task_category = task_category.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                task_category.set(input.value());
                            }
                        })}
                    />
                    <label for="description">{ "Descrição:" }</label>
                    <input 
                        type="text" 
                        id="description" 
                        name="description" 
                        required=true 
                        value={(*task_description).clone()}
                        oninput={Callback::from({
                            let task_description = task_description.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                task_description.set(input.value());
                            }
                        })}
                    />
                    <label for="begin_date">{ "Data de Início:" }</label>
                    <input 
                        type="datetime-local" 
                        id="begin_date" 
                        name="begin_date" 
                        required=true 
                        value={(*task_begin_date).clone()}
                        oninput={Callback::from({
                            let task_begin_date = task_begin_date.clone();
                            move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                task_begin_date.set(input.value());
                            }
                        })}
                    />

                    <label for="type">{ "Tipo:" }</label>
                    <select 
                        id="type" 
                        name="type" 
                        required=true 
                        value={(*task_type).clone()}
                        onchange={Callback::from({
                            let task_type = task_type.clone();
                            move |e: Event| {
                                let select: HtmlInputElement = e.target_unchecked_into();
                                task_type.set(select.value());
                            }
                        })}
                    >
                        <option value="MeiaHora">{ "Meia Hora" }</option>
                        <option value="UmaHora">{ "Uma Hora" }</option>
                        <option value="Manhã">{ "Manhã" }</option>
                        <option value="Tarde">{ "Tarde" }</option>
                        <option value="Noite">{ "Noite" }</option>
                    </select>
                    <button type="submit">{ "Adicionar" }</button>
                </form>
            </div>
        </>
    }
}
