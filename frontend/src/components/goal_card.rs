use yew::{function_component, html, Html, Properties, Callback, MouseEvent, classes, use_state, InputEvent, TargetCast};
use web_sys::Event as ChangeEvent;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use wasm_bindgen_futures::spawn_local;
use crate::services::goal_service::{GoalDto, update_goal};

#[derive(Properties, PartialEq)]
pub struct GoalCardProps {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub status: String,
    pub goal_type: String,
    pub date_start: String,
    pub date_end: String,
    pub days_remaining: i64,
    pub progress_percentage: f32,
    pub on_goal_delete: Callback<i32>,
    #[prop_or_default]
    pub on_goal_updated: Option<Callback<()>>,
}

#[function_component(GoalCard)]
pub fn goal_card(props: &GoalCardProps) -> Html {
    let is_editing = use_state(|| false);
    let edit_name = use_state(|| props.name.clone());
    let edit_description = use_state(|| props.description.clone().unwrap_or_default());
    let edit_category = use_state(|| props.category.clone().unwrap_or_default());
    let edit_status = use_state(|| props.status.clone());
    let edit_goal_type = use_state(|| props.goal_type.clone());

    let on_edit_click = {
        let is_editing = is_editing.clone();
        let edit_name = edit_name.clone();
        let edit_description = edit_description.clone();
        let edit_category = edit_category.clone();
        let edit_status = edit_status.clone();
        let edit_goal_type = edit_goal_type.clone();
        let name = props.name.clone();
        let description = props.description.clone();
        let category = props.category.clone();
        let status = props.status.clone();
        let goal_type = props.goal_type.clone();
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            
            if *is_editing {
                // Cancelar edição - restaurar valores originais
                edit_name.set(name.clone());
                edit_description.set(description.clone().unwrap_or_default());
                edit_category.set(category.clone().unwrap_or_default());
                edit_status.set(status.clone());
                edit_goal_type.set(goal_type.clone());
                is_editing.set(false);
            } else {
                // Iniciar edição
                is_editing.set(true);
            }
        })
    };

    let on_save_click = {
        let is_editing = is_editing.clone();
        let edit_name = edit_name.clone();
        let edit_description = edit_description.clone();
        let edit_category = edit_category.clone();
        let edit_status = edit_status.clone();
        let edit_goal_type = edit_goal_type.clone();
        let on_goal_updated = props.on_goal_updated.clone();
        let goal_id = props.id;
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            
            let name = (*edit_name).clone();
            let description = (*edit_description).clone();
            let category = (*edit_category).clone();
            let status = (*edit_status).clone();
            let goal_type = (*edit_goal_type).clone();
            let is_editing = is_editing.clone();
            let on_goal_updated = on_goal_updated.clone();
            
            spawn_local(async move {
                let goal_dto = GoalDto {
                    name,
                    description: if description.trim().is_empty() { None } else { Some(description) },
                    category: if category.trim().is_empty() { None } else { Some(category) },
                    status,
                    goal_type,
                };
                
                match update_goal(goal_id, goal_dto).await {
                    Ok(_) => {
                        is_editing.set(false);
                        if let Some(callback) = on_goal_updated {
                            callback.emit(());
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to update goal: {}", e).into());
                    }
                }
            });
        })
    };

    let goal_id = props.id;
    let on_delete_click = {
        let on_goal_delete = props.on_goal_delete.clone();
        let goal_name = props.name.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            
            let confirm = web_sys::window()
                .unwrap()
                .confirm_with_message(&format!("Tem certeza que deseja excluir a meta '{}'?", goal_name))
                .unwrap();
                
            if confirm {
                on_goal_delete.emit(goal_id);
            }
        })
    };

    // Handlers para inputs de edição
    let on_name_change = {
        let edit_name = edit_name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                edit_name.set(target.value());
            }
        })
    };

    let on_description_change = {
        let edit_description = edit_description.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                edit_description.set(target.value());
            }
        })
    };

    let on_category_change = {
        let edit_category = edit_category.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                edit_category.set(target.value());
            }
        })
    };

    let on_status_change = {
        let edit_status = edit_status.clone();
        Callback::from(move |e: ChangeEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                edit_status.set(target.value());
            }
        })
    };

    let on_goal_type_change = {
        let edit_goal_type = edit_goal_type.clone();
        Callback::from(move |e: ChangeEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                edit_goal_type.set(target.value());
            }
        })
    };

    let status_class = props.status.to_lowercase().replace(" ", "-");
    let goal_type_class = props.goal_type.to_lowercase();

    html! {
        <div class="goal-card">
            <div class="goal-header">
                {if *is_editing {
                    html! {
                        <input 
                            type="text" 
                            class="goal-title-edit" 
                            value={(*edit_name).clone()} 
                            oninput={on_name_change}
                        />
                    }
                } else {
                    html! { <h3 class="goal-title">{ &props.name }</h3> }
                }}
                
                <div class="goal-actions">
                    {if *is_editing {
                        html! {
                            <>
                                <button class="save-button" onclick={on_save_click}>{ "Salvar" }</button>
                                <button class="cancel-button" onclick={on_edit_click}>{ "Cancelar" }</button>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <button class="edit-button" onclick={on_edit_click}>{ "Editar" }</button>
                                <button class="delete-button" onclick={on_delete_click}>{ "Excluir" }</button>
                            </>
                        }
                    }}
                </div>
            </div>
            
            <div class="goal-body">
                <div class="goal-meta">
                    {if *is_editing {
                        html! {
                            <select class="goal-type-edit" value={(*edit_goal_type).clone()} onchange={on_goal_type_change}>
                                <option value="weekly">{ "Semanal" }</option>
                                <option value="monthly">{ "Mensal" }</option>
                                <option value="annual">{ "Anual" }</option>
                            </select>
                        }
                    } else {
                        html! { <span class={classes!("goal-type", goal_type_class)}>{ &props.goal_type }</span> }
                    }}
                    
                    {if *is_editing {
                        html! {
                            <input 
                                type="text" 
                                class="goal-category-edit" 
                                placeholder="Categoria (opcional)"
                                value={(*edit_category).clone()} 
                                oninput={on_category_change}
                            />
                        }
                    } else {
                        if let Some(ref category) = props.category {
                            html! { <span class="goal-category">{ category }</span> }
                        } else {
                            html! {}
                        }
                    }}
                </div>

                {if *is_editing {
                    html! {
                        <input 
                            type="text" 
                            class="goal-description-edit" 
                            placeholder="Descrição (opcional)"
                            value={(*edit_description).clone()} 
                            oninput={on_description_change}
                        />
                    }
                } else {
                    if let Some(ref description) = props.description {
                        html! { <p class="goal-description">{ description }</p> }
                    } else {
                        html! { <p class="goal-description empty">{ "Sem descrição" }</p> }
                    }
                }}

                <div class="goal-progress">
                    <div class="progress-info">
                        <span class="progress-percentage">{ format!("{:.0}%", props.progress_percentage) }</span>
                        <span class="days-remaining">
                            { if props.days_remaining > 0 {
                                format!("{} dias restantes", props.days_remaining)
                            } else if props.days_remaining == 0 {
                                "Último dia!".to_string()
                            } else {
                                format!("{} dias em atraso", -props.days_remaining)
                            }}
                        </span>
                    </div>
                    <div class="progress-bar">
                        <div 
                            class="progress-fill"
                            style={format!("width: {}%", props.progress_percentage.min(100.0))}
                        />
                    </div>
                </div>
            </div>
            
            <div class="goal-footer">
                <div class="date-range">
                    <span class="date-label">{ "Período:" }</span>
                    <span class="date-value">{ format!("{} a {}", props.date_start, props.date_end) }</span>
                </div>
                <div class="status-indicator-container">
                    <span class="date-label">{ "Status:" }</span>
                    {if *is_editing {
                        html! {
                            <select class="status-edit" value={(*edit_status).clone()} onchange={on_status_change}>
                                <option value="NotStarted">{ "Não Iniciado" }</option>
                                <option value="InProgress">{ "Em Progresso" }</option>
                                <option value="Completed">{ "Concluído" }</option>
                                <option value="Paused">{ "Pausado" }</option>
                                <option value="Cancelled">{ "Cancelado" }</option>
                            </select>
                        }
                    } else {
                        html! { <span class={classes!("status-indicator", status_class)}>{ &props.status }</span> }
                    }}
                </div>
            </div>
        </div>
    }
}