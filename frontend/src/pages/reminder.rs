// use yew::prelude::*;
use web_sys::HtmlInputElement;
use chrono::{DateTime, Utc, NaiveDateTime};
use wasm_bindgen_futures::spawn_local;

use crate::services::reminder_service::{ReminderDto, create_reminder, ReminderResult};

#[function_component(ReminderPage)]
pub fn reminder_page() -> Html {
    let name = use_state(|| String::new());
    let date_end = use_state(|| String::new());
    let category = use_state(|| String::new());

    // Lista de reminders exibidos
    let reminders = use_state(|| Vec::<ReminderDto>::new());

    // Handlers de input
    let on_name_input = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                name.set(input.value());
            }
        })
    };

    let on_date_input = {
        let date_end = date_end.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                date_end.set(input.value());
            }
        })
    };

    let on_category_input = {
        let category = category.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                category.set(input.value());
            }
        })
    };

    // Submit
    let onsubmit = {
        let name = name.clone();
        let date_end = date_end.clone();
        let category = category.clone();
        let reminders = reminders.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let name_val = (*name).clone();
            let date_val = (*date_end).clone();
            let category_val = (*category).clone();

            if name_val.is_empty() || date_val.is_empty() || category_val.is_empty() {
                web_sys::console::error_1(&"All fields are required".into());
                return;
            }

            // Converte "YYYY-MM-DDTHH:MM" -> DateTime<Utc>
            match NaiveDateTime::parse_from_str(&date_val, "%Y-%m-%dT%H:%M") {
                Ok(naive) => {
                    let utc: DateTime<Utc> = DateTime::<Utc>::from_utc(naive, Utc);
                    let reminder_dto = ReminderDto {
                        name: name_val.clone(),
                        date_end: utc,
                        category: category_val.clone(),
                    };

                    // Chama o service para salvar no backend
                    spawn_local({
                        let reminders = reminders.clone();
                        async move {
                            match create_reminder(&reminder_dto).await {
                                ReminderResult::Success(saved) => {
                                    web_sys::console::log_1(&format!("Reminder saved: {:?}", saved).into());

                                    // Atualiza lista local
                                    reminders.set({
                                        let mut list = (*reminders).clone();
                                        list.push(saved);
                                        list
                                    });
                                }
                                ReminderResult::InvalidFields => {
                                    web_sys::console::error_1(&"Invalid fields provided".into());
                                }
                                ReminderResult::NetworkError(err) => {
                                    web_sys::console::error_1(&format!("Network error: {}", err).into());
                                }
                            }
                        }
                    });

                    // Limpa inputs
                    name.set(String::new());
                    date_end.set(String::new());
                    category.set(String::new());
                }
                Err(_) => {
                    web_sys::console::error_1(&"Invalid date format".into());
                }
            }
        })
    };

    html! {
        <div class="reminder-page">
            <h1>{ "Crie um lembrete" }</h1>
            <form onsubmit={onsubmit}>
                <label for="name">{ "Nome:" }</label>
                <input
                    id="name"
                    type="text"
                    value={(*name).clone()}
                    oninput={on_name_input}
                    required=true
                />

                <label for="date_end">{ "Data de Término:" }</label>
                <input
                    id="date_end"
                    type="datetime-local"
                    value={(*date_end).clone()}
                    oninput={on_date_input}
                    required=true
                />

                <label for="category">{ "Categoria:" }</label>
                <input
                    id="category"
                    type="text"
                    value={(*category).clone()}
                    oninput={on_category_input}
                    required=true
                />

                <button type="submit">{ "Criar" }</button>
            </form>
            /* 
            <h2>{ "Lembretes:" }</h2>
            <ul>
                { for reminders.iter().map(|r| html!{
                    <li>
                        <b>{ &r.name }</b>
                        { " | " }{ r.date_end.to_rfc3339() }
                        { " | " }{ &r.category }
                    </li>
                }) }
            </ul>  */ 
        </div>
    }
    
}
