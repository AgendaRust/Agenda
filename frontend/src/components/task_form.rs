use yew::{function_component, html, use_state, Callback, Html, InputEvent, MouseEvent, Properties, TargetCast};
use web_sys::HtmlInputElement;
use chrono::NaiveDate;
use crate::types::TaskDuration;

#[derive(Properties, PartialEq)]
pub struct TaskFormProps {
    pub visible: bool,
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
    pub selected_date: NaiveDate,
}

#[function_component(TaskForm)]
pub fn task_form(props: &TaskFormProps) -> Html {

    let _task_title = use_state(|| String::new());
    let _task_category = use_state(|| String::new());
    let _task_description = use_state(|| String::new());
    let task_hour = use_state(|| 9u32);
    let task_minute = use_state(|| 0u32);
    let _task_type = use_state(|| TaskDuration::default());

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
                <div class="task-form">
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
                    
                    // Hidden input for begin_date (combined date + time)
                    <input 
                        type="hidden" 
                        name="begin_date" 
                        value={begin_date.clone()}
                    />
                    
                    // Debug display (you can remove this later)
                    <div style="font-size: 0.8rem; color: #666; margin-bottom: 10px;">
                        { format!("Data/Hora: {}", begin_date) }
                    </div>
                    
                    <label for="title">{ "Nova task:" }</label>
                    <input 
                        type="text" 
                        id="title" 
                        name="title" 
                        minlength="3" 
                        required=true 
                        placeholder="Digite o título da task"
                    />
                    
                    <label for="category">{ "Categoria:" }</label>
                    <input 
                        type="text" 
                        id="category" 
                        name="category" 
                        minlength="5" 
                        required=true 
                        placeholder="Digite a categoria"
                    />
                    
                    <label for="description">{ "Descrição:" }</label>
                    <textarea 
                        id="description" 
                        name="description" 
                        required=true 
                        placeholder="Digite a descrição"
                        rows="3"
                    ></textarea>

                    <label for="type">{ "Tipo:" }</label>
                    <select 
                        id="type" 
                        name="type" 
                        required=true 
                    >
                        {
                            TaskDuration::all().iter().map(|duration| {
                                html! {
                                    <option value={duration.value()}>
                                        { duration.display_name() }
                                    </option>
                                }
                            }).collect::<Html>()
                        }
                    </select>
                    
                    <button type="submit">{"Add Task"}</button>
                    <button type="button" onclick={on_close}>{"Cancel"}</button>
                </div>
            </div>
    }
}
}