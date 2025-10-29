use yew::{function_component, html, use_state, Callback, Html, InputEvent, MouseEvent, Properties, TargetCast};
use web_sys::HtmlInputElement;
use chrono::{NaiveDate, NaiveDateTime, Utc, TimeZone, DateTime};
use wasm_bindgen_futures::spawn_local;

use crate::services::reminder_service::{ReminderDto, create_reminder, ReminderResult};
use crate::types::reminder::Reminder;

#[derive(Properties, PartialEq)]
pub struct ReminderFormProps {
    pub visible: bool,
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
    #[prop_or_default]
    pub on_reminder_created: Option<Callback<Reminder>>,
    // pub current_day: Option<NaiveDate>,
}


#[function_component(ReminderForm)]
pub fn reminder_form(props: &ReminderFormProps) -> Html {
    let reminder_name = use_state(|| String::new());
    let reminder_category = use_state(|| String::new());
    let reminder_date = use_state(|| String::new());
    let form_status = use_state(|| String::new());

    let on_name_change = {
        let reminder_name = reminder_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            reminder_name.set(input.value());
        })
    };

    let on_category_change = {
        let reminder_category = reminder_category.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            reminder_category.set(input.value());
        })
    };

    let on_date_change = {
        let reminder_date = reminder_date.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            reminder_date.set(input.value());
        })
    };

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(cb) = &on_close {
                cb.emit(());
            }
        })
    };

    let on_create = {
        let reminder_name = reminder_name.clone();
        let reminder_category = reminder_category.clone();
        let reminder_date = reminder_date.clone();
        let form_status = form_status.clone();
        let on_close = props.on_close.clone();
        let on_reminder_created = props.on_reminder_created.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let reminder_name = reminder_name.clone();
            let reminder_category = reminder_category.clone();
            let reminder_date = reminder_date.clone();
            let form_status = form_status.clone();
            let on_close = on_close.clone();
            let on_reminder_created = on_reminder_created.clone();

            spawn_local(async move {
                let parsed_date: Option<DateTime<Utc>> = NaiveDate::parse_from_str(&*reminder_date, "%Y-%m-%d")
                    .map(|d| NaiveDateTime::new(d, chrono::NaiveTime::from_hms_opt(0,0,0).unwrap()))
                    .ok()
                    .map(|naive| Utc.from_utc_datetime(&naive));

                if parsed_date.is_none() {
                    form_status.set("error".to_string());
                    return;
                }

                let reminder_info = ReminderDto {
                    name: (*reminder_name).clone(),
                    category: (*reminder_category).clone(),
                    date_end: parsed_date.unwrap(),
                };

                let result = create_reminder(&reminder_info).await;
                match result {
                    ReminderResult::Success(rem) => {
                        web_sys::console::log_1(&format!("Lembrete criado com sucesso: {:?}", rem).into());
                        form_status.set("success".to_string());

                        if let Some(cb) = &on_reminder_created {
                            cb.emit(rem);
                        }

                        let reminder_name = reminder_name.clone();
                        let reminder_category = reminder_category.clone();
                        let reminder_date = reminder_date.clone();
                        let form_status = form_status.clone();
                        let on_close = on_close.clone();

                        wasm_bindgen_futures::spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(1500).await;
                            reminder_name.set(String::new());
                            reminder_category.set(String::new());
                            reminder_date.set(String::new());
                            form_status.set(String::new());

                            if let Some(cb) = &on_close {
                                cb.emit(());
                            }
                        });
                    }
                    ReminderResult::InvalidFields => {
                        form_status.set("error".to_string());
                    }
                    ReminderResult::NetworkError(err) => {
                        web_sys::console::log_1(&format!("Erro de rede: {}", err).into());
                        form_status.set("error".to_string());
                    }
                }
            });
        })
    };

    html! {
        if props.visible {
            <div class="reminder-popup">
                <div class={format!("reminder-form {}", (*form_status).clone())}>
                    <div class="reminder-form-header">
                        <div class="title-text">{ "Criar Lembrete" }</div>
                        <div class="window-controls">
                            <div class="control-button minimize"></div>
                            <div class="control-button maximize"></div>
                            <div class="control-button close" onclick={on_close.clone()}></div>
                        </div>
                    </div>

                    <div class="reminder-form-content">
                        if !form_status.is_empty() {
                            <div class={format!("status-message {}", (*form_status).clone())}>
                                if *form_status == "success" {
                                    { "✓ Lembrete criado com sucesso!" }
                                } else if *form_status == "error" {
                                    { "✗ Falha ao criar lembrete. Verifique os campos." }
                                }
                            </div>
                        }

                        <div>
                            <label for="name">{ "Nome:" }</label>
                            <input
                                type="text"
                                id="name"
                                name="name"
                                required=true
                                placeholder="Digite o nome do lembrete"
                                value={(*reminder_name).clone()}
                                oninput={on_name_change}
                                required=true
                            />
                        </div>

                        <div>
                            <label for="category">{ "Categoria:" }</label>
                            <input
                                type="text"
                                id="category"
                                name="category"
                                required=true
                                placeholder="Digite a categoria"
                                value={(*reminder_category).clone()}
                                oninput={on_category_change}
                            />
                        </div>

                        <div>
                            <label for="date_end">{ "Data de término:" }</label>
                            <input
                                type="date"
                                id="date_end"
                                name="date_end"
                                required=true
                                value={(*reminder_date).clone()}
                                oninput={on_date_change}
                            />
                        </div>

                        <div class="button-container">
                            <button type="submit" onclick={on_create}>{ "Criar Lembrete" }</button>
                            <button type="button" onclick={on_close}>{ "Cancelar" }</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
