use yew::{function_component, html, Html, Properties, Callback, MouseEvent, use_state, InputEvent, TargetCast};
use chrono::{DateTime, Utc, Local, TimeZone};
use crate::services::reminder_service::{ReminderUpdateDto, update_reminder};

fn format_reminder_date(date_end: &DateTime<Utc>) -> String {
    let local_date = date_end.with_timezone(&Local::now().timezone());
    local_date.format("%Y-%m-%d").to_string()
}

fn format_reminder_time(date_end: &DateTime<Utc>) -> String {
    let local_date = date_end.with_timezone(&Local::now().timezone());
    local_date.format("%H:%M").to_string()
}

#[derive(Properties, PartialEq)]
pub struct ReminderCardProps {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub date_end: DateTime<Utc>,
    pub on_reminder_delete: Callback<i32>,
    #[prop_or_default]
    pub on_reminder_update: Option<Callback<(i32, String, String, String)>>,
}

#[function_component(ReminderCard)]
pub fn reminder_card(props: &ReminderCardProps) -> Html {
    let is_editing = use_state(|| false);
    let edit_name = use_state(|| props.name.clone());
    let edit_category = use_state(|| props.category.clone());
    let edit_date_end = use_state(|| format_reminder_date(&props.date_end));

    let on_edit_click = {
        let is_editing = is_editing.clone();
        let edit_name = edit_name.clone();
        let edit_category = edit_category.clone();
        let edit_date_end = edit_date_end.clone();
        let props_name = props.name.clone();
        let props_category = props.category.clone();
        let props_date_end = format_reminder_date(&props.date_end);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            is_editing.set(true);
            edit_name.set(props_name.clone());
            edit_category.set(props_category.clone());
            edit_date_end.set(props_date_end.clone());
        })
    };

    let reminder_id = props.id;
    let on_delete_click = {
        let on_reminder_delete = props.on_reminder_delete.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            web_sys::console::log_1(&"Delete reminder clicked".into());
            on_reminder_delete.emit(reminder_id);
        })
    };

    let on_name_input = {
        let edit_name = edit_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            edit_name.set(input.value());
        })
    };
    let on_category_input = {
        let edit_category = edit_category.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            edit_category.set(input.value());
        })
    };
    let on_date_input = {
        let edit_date_end = edit_date_end.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            edit_date_end.set(input.value());
        })
    };

    let on_save_click = {
        let is_editing = is_editing.clone();
        let edit_name = edit_name.clone();
        let edit_category = edit_category.clone();
        let edit_date_end = edit_date_end.clone();
        let on_reminder_update = props.on_reminder_update.clone();
        let reminder_id = props.id;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let name = (*edit_name).clone();
            let category = (*edit_category).clone();
            let date_end = (*edit_date_end).clone();
            let dto = ReminderUpdateDto {
                name: name.clone(),
                category: category.clone(),
                date_end: Local.ymd(date_end[0..4].parse().unwrap(), date_end[5..7].parse().unwrap(), date_end[8..10].parse().unwrap())
                    .and_hms(0, 0, 0)
                    .with_timezone(&Utc),
            };
            let date_end_str = date_end.clone();
            let on_reminder_update = on_reminder_update.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = update_reminder(reminder_id as u32, dto).await;
                match result {
                    Ok(_) => {
                        if let Some(cb) = &on_reminder_update {
                            cb.emit((reminder_id, name, category, date_end_str));
                        }
                    }
                    Err(err) => {
                        web_sys::console::log_1(&format!("Failed to update reminder: {}", err).into());
                    }
                }
            });
            is_editing.set(false);
        })
    };

    let on_cancel_click = {
        let is_editing = is_editing.clone();
        let edit_name = edit_name.clone();
        let edit_category = edit_category.clone();
        let edit_date_end = edit_date_end.clone();
        let props_name = props.name.clone();
        let props_category = props.category.clone();
        let props_date_end = format_reminder_date(&props.date_end);
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            is_editing.set(false);
            edit_name.set(props_name.clone());
            edit_category.set(props_category.clone());
            edit_date_end.set(props_date_end.clone());
        })
    };

    html! {
        <div class="reminder-card">
            <div class="reminder-header">
                if *is_editing {
                    <input 
                        class="reminder-title-input"
                        type="text"
                        value={(*edit_name).clone()}
                        oninput={on_name_input}
                    />
                } else {
                    <h3 class="reminder-title">{ &props.name }</h3>
                }
                <div class="reminder-actions">
                    if *is_editing {
                        <button class="save-button" onclick={on_save_click}>{ "Salvar" }</button>
                        <button class="cancel-button" onclick={on_cancel_click}>{ "Cancelar" }</button>
                    } else {
                        <button class="edit-button" onclick={on_edit_click}>{ "Editar" }</button>
                        <button class="delete-button" onclick={on_delete_click}>{ "Excluir" }</button>
                    }
                </div>
            </div>
            <div class="reminder-body">
                if *is_editing {
                    <input 
                        class="reminder-category-input"
                        type="text"
                        value={(*edit_category).clone()}
                        oninput={on_category_input}
                    />
                    <input 
                        class="reminder-date-input"
                        type="date"
                        value={(*edit_date_end).clone()}
                        oninput={on_date_input}
                    />
                } else {
                    <p class="reminder-category">{ &props.category }</p>
                    <div class="reminder-datetime">
                        <span class="reminder-date">{ format!("Due Date: {}", format_reminder_date(&props.date_end)) }</span>
                        <span class="reminder-time">{ format_reminder_time(&props.date_end) }</span>
                    </div>
                }
            </div>
        </div>
    }
}
