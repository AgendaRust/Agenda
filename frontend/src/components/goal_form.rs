// src/components/goal_form.rs

use yew::{function_component, html, use_state, use_effect_with, Callback, Html, InputEvent, MouseEvent, Properties, TargetCast, classes};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use wasm_bindgen_futures::spawn_local;

use crate::types::goal::Goal;
// Importe tanto create_goal quanto update_goal do seu serviço
use crate::services::goal_service::{GoalDto, create_goal, update_goal};

// Seu componente Windows98Select está ótimo, ele será mantido como está.
#[derive(Properties, PartialEq, Clone)]
pub struct Windows98SelectProps<T> where T: PartialEq + Clone + 'static {
    pub value: T,
    pub on_change: Callback<T>,
    pub options: Vec<(T, String)>,
}

#[function_component(Windows98Select)]
pub fn windows98_select<T>(props: &Windows98SelectProps<T>) -> Html where T: PartialEq + Clone + 'static {
    let is_open = use_state(|| false);

    let toggle_dropdown = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_open.set(!*is_open);
        })
    };

    let select_option = {
        let is_open = is_open.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |value: T| {
            on_change.emit(value);
            is_open.set(false);
        })
    };

    let current_label = props.options.iter()
        .find(|(val, _)| *val == props.value)
        .map(|(_, label)| label.clone())
        .unwrap_or_else(|| "Selecione".to_string());

    html! {
        <div class="win98-select-container">
            <div class="win98-select-button" onclick={toggle_dropdown}>
                <span class="win98-select-text">{current_label}</span>
                <span class="win98-select-arrow">{"▼"}</span>
            </div>

            if *is_open {
                <div class="win98-select-dropdown">
                    { for props.options.iter().map(|(value, label)| {
                        let is_selected = value.clone() == props.value;
                        let select_option = select_option.clone();
                        let value = value.clone();

                        html! {
                            <div
                                class={classes!("win98-select-option", is_selected.then_some("selected"))}
                                onclick={Callback::from(move |_| select_option.emit(value.clone()))}
                            >
                                {label}
                            </div>
                        }
                    }) }
                </div>
            }
        </div>
    }
}


#[derive(Properties, PartialEq, Clone)]
pub struct GoalFormProps {
    pub visible: bool,
    #[prop_or_default]
    pub goal_to_edit: Option<Goal>, // Se Some(goal), edita. Se None, cria.
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,
    #[prop_or_default]
    pub on_save: Option<Callback<Goal>>, // Callback único para sucesso.
}

#[function_component(GoalForm)]
pub fn goal_form(props: &GoalFormProps) -> Html {
    let is_edit_mode = props.goal_to_edit.is_some();

    // Estados dos campos do formulário
    let goal_name = use_state(String::new);
    let goal_description = use_state(String::new);
    let goal_category = use_state(String::new);
    let goal_status = use_state(String::new);
    let goal_type = use_state(String::new);
    let form_status = use_state(String::new);

    {
        let props_clone = props.clone();
        let states = (goal_name.clone(), goal_description.clone(), goal_category.clone(), goal_status.clone(), goal_type.clone());

        use_effect_with(props_clone, move |props| {
            if let Some(goal) = &props.goal_to_edit {
                states.0.set(goal.name.clone());
                states.1.set(goal.description.clone().unwrap_or_default());
                states.2.set(goal.category.clone().unwrap_or_default());
                states.3.set(goal.status.clone());
                states.4.set(goal.goal_type.clone());
            } else {
                states.0.set(String::new());
                states.1.set(String::new());
                states.2.set(String::new());
                states.3.set("Em andamento".to_string());
                states.4.set("monthly".to_string());
            }
        });
    }

    let on_input_change = |state: yew::UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                state.set(input.value());
            }
        })
    };
    let on_textarea_change = |state: yew::UseStateHandle<String>| {
        Callback::from(move |e: InputEvent| {
            if let Some(textarea) = e.target_dyn_into::<HtmlTextAreaElement>() {
                state.set(textarea.value());
            }
        })
    };

    let on_save_click = {
        let states = (goal_name.clone(), goal_description.clone(), goal_category.clone(), goal_status.clone(), goal_type.clone(), form_status.clone());
        let on_close = props.on_close.clone();
        let on_save = props.on_save.clone();
        let goal_to_edit = props.goal_to_edit.clone();

        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let (name, desc, cat, status, type_, form_stat) = states.clone();
            let on_close = on_close.clone();
            let on_save = on_save.clone();
            let goal_to_edit = goal_to_edit.clone();

            spawn_local(async move {
                if name.trim().is_empty() {
                    form_stat.set("error".to_string()); return;
                }

                let goal_info = GoalDto {
                    name: (*name).clone(),
                    description: Some((*desc).clone()).filter(|s| !s.trim().is_empty()),
                    category: Some((*cat).clone()).filter(|s| !s.trim().is_empty()),
                    status: (*status).clone(),
                    goal_type: (*type_).clone(),
                };

                let result = if let Some(goal) = goal_to_edit {
                    update_goal(goal.id, goal_info).await
                } else {
                    create_goal(goal_info).await
                };

                match result {
                    Ok(saved_goal) => {
                        form_stat.set("success".to_string());
                        if let Some(cb) = &on_save { cb.emit(saved_goal); }

                        let on_close = on_close.clone();
                        spawn_local(async move {
                            gloo_timers::future::TimeoutFuture::new(1500).await;
                            if let Some(cb) = on_close { cb.emit(()); }
                        });
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Falha ao salvar meta: {}", err).into());
                        form_stat.set("error".to_string());
                    }
                }
            });
        })
    };

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_: MouseEvent| { if let Some(cb) = &on_close { cb.emit(()) } })
    };

    let title_text = if is_edit_mode { "Editar Meta" } else { "Criar Meta" };
    let button_text = if is_edit_mode { "Salvar Alterações" } else { "Criar Meta" };
    let success_message = if is_edit_mode { "✓ Meta atualizada!" } else { "✓ Meta criada!" };
    let error_message = if is_edit_mode { "✗ Falha ao atualizar." } else { "✗ Falha ao criar." };

    html! {
        if props.visible {
            <div class="task-popup">
                <div class={format!("task-form {}", (*form_status).clone())}>
                    <div class="task-form-header">
                        <div class="title-text">{title_text}</div>
                        <div class="window-controls">
                            <div class="control-button minimize"></div>
                            <div class="control-button maximize"></div>
                            <div class="control-button close" onclick={on_close.clone()}></div>
                        </div>
                    </div>

                    <div class="task-form-content">
                        if !form_status.is_empty() {
                            <div class={format!("status-message {}", (*form_status).clone())}>
                                { if *form_status == "success" { success_message } else { error_message }}
                            </div>
                        }

                        // Coluna da Esquerda
                        <div>
                            <label for="name">{ "Nome da Meta:" }</label>
                            <input type="text" id="name" required=true value={(*goal_name).clone()} oninput={on_input_change(goal_name.clone())} />
                        </div>

                        // Coluna da Direita
                        <div>
                            <label for="category">{ "Categoria:" }</label>
                            <input type="text" id="category" value={(*goal_category).clone()} oninput={on_input_change(goal_category.clone())} />

                            <label for="goal_type">{ "Tipo de Meta:" }</label>
                            <Windows98Select<String>
                                value={(*goal_type).clone()}
                                on_change={Callback::from(move |val| goal_type.set(val))}
                                options={vec![
                                    ("weekly".to_string(), "Semanal".to_string()),
                                    ("monthly".to_string(), "Mensal".to_string()),
                                    ("annual".to_string(), "Anual".to_string()),
                                ]}
                            />
                        </div>

                        <div class="full-width">
                            <label for="description">{ "Descrição:" }</label>
                            <textarea id="description" rows="3" value={(*goal_description).clone()} oninput={on_textarea_change(goal_description.clone())}></textarea>
                        </div>

                        <div class="button-container">
                            <button type="submit" onclick={on_save_click}>{button_text}</button>
                            <button type="button" onclick={on_close}>{"Cancelar"}</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}