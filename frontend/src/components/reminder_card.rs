use yew::{function_component, html, Html, Properties, Callback, MouseEvent};
use chrono::{DateTime, Utc, Local, TimeZone};

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
}

#[function_component(ReminderCard)]
pub fn reminder_card(props: &ReminderCardProps) -> Html {
    let on_edit_click = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            web_sys::console::log_1(&"Edit reminder clicked".into());
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

    html! {
        <div class="reminder-card">
            <div class="reminder-header">
                <h3 class="reminder-title">{ &props.name }</h3>
                <div class="reminder-actions">
                    <button class="edit-button" onclick={on_edit_click}>{ "Editar" }</button>
                    <button class="delete-button" onclick={on_delete_click}>{ "Excluir" }</button>
                </div>
            </div>
            <div class="reminder-body">
                <p class="reminder-category">{ &props.category }</p>
                <div class="reminder-datetime">
                    <span class="reminder-date">{ format!("Due Date: {}", format_reminder_date(&props.date_end)) }</span>
                    <span class="reminder-time">{ format_reminder_time(&props.date_end) }</span>
                </div>
            </div>
        </div>
    }
}
