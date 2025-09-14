// frontend/src/components/goals_list.rs

use yew::{function_component, html, use_state, use_effect, Callback, Html, Properties};
use wasm_bindgen_futures::spawn_local;

// Importe os tipos e serviços necessários
use crate::types::goal::Goal;
use crate::services::goal_service::{get_all_goals, delete_goal, update_goal, GoalDto};

// Importe os componentes filhos que este componente vai usar
use crate::components::goal_form::GoalForm;
use crate::components::goal_card::GoalCard;

#[derive(Properties, PartialEq)]
pub struct GoalsListProps {}

#[function_component(GoalsList)]
pub fn goals_list(_props: &GoalsListProps) -> Html {
    let goals = use_state(Vec::new);
    let show_form = use_state(|| false);
    let goal_to_edit = use_state(|| None::<Goal>);
    let error_message = use_state(String::new);

    {
        let goals = goals.clone();
        let error_message = error_message.clone();
        use_effect(move || {
            let goals = goals.clone();
            let error_message = error_message.clone();
            spawn_local(async move {
                match get_all_goals().await {
                    Ok(fetched_goals) => {
                        goals.set(fetched_goals);
                    }
                    Err(err) => {
                        error_message.set(format!("Erro ao carregar metas: {}", err));
                    }
                }
            });
        });
    }

    let reload_goals = {
        let goals = goals.clone();
        let error_message = error_message.clone();
        Callback::from(move |_| {
            let goals = goals.clone();
            let error_message = error_message.clone();
            spawn_local(async move {
                match get_all_goals().await {
                    Ok(fetched_goals) => {
                        goals.set(fetched_goals);
                    }
                    Err(err) => {
                        error_message.set(format!("Erro ao recarregar metas: {}", err));
                    }
                }
            });
        })
    };

    let on_new_goal_click = {
        let show_form = show_form.clone();
        let goal_to_edit = goal_to_edit.clone();
        Callback::from(move |_| {
            goal_to_edit.set(None);
            show_form.set(true);
        })
    };

    let on_edit_goal = {
        let show_form = show_form.clone();
        let goal_to_edit = goal_to_edit.clone();
        Callback::from(move |goal: Goal| {
            goal_to_edit.set(Some(goal));
            show_form.set(true);
        })
    };

    let on_delete_goal = {
        let reload_goals = reload_goals.clone();
        Callback::from(move |goal_id: i32| {
            let reload_goals = reload_goals.clone();
            spawn_local(async move {
                if delete_goal(goal_id).await.is_ok() {
                    reload_goals.emit(());
                } else {
                }
            });
        })
    };

    let on_close_form = {
        let show_form = show_form.clone();
        Callback::from(move |_| {
            show_form.set(false);
        })
    };

    let on_save_goal = {
        let show_form = show_form.clone();
        let reload_goals = reload_goals.clone();
        Callback::from(move |_goal: Goal| {
            show_form.set(false);
            reload_goals.emit(());
        })
    };

    let on_goal_status_update = {
        let goals = goals.clone();
        let reload_goals = reload_goals.clone();
        Callback::from(move |(goal_id, new_status): (i32, String)| {
            let goals = goals.clone();
            let reload_goals = reload_goals.clone();

            if let Some(goal_to_update) = (*goals).iter().find(|g| g.id == goal_id).cloned() {
                spawn_local(async move {
                    let goal_dto = GoalDto {
                        name: goal_to_update.name,
                        description: goal_to_update.description,
                        category: goal_to_update.category,
                        goal_type: goal_to_update.goal_type,
                    };

                    match update_goal(goal_id, goal_dto).await {
                        Ok(_) => {
                            reload_goals.emit(());
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("Falha ao atualizar status da meta: {}", e).into());
                        }
                    }
                });
            }
        })
    };

    html! {
        <div class="goals-list-container">
            <div class="metas-actions">
                <button class="win98-button" onclick={on_new_goal_click}>
                    { "Nova Meta" }
                </button>
            </div>

            if !(*error_message).is_empty() {
                <p class="error-message">{(*error_message).clone()}</p>
            }

            <div class="goals-scroll-view">
            {
                for goals.iter().map(|goal| {
                    let on_edit_goal = on_edit_goal.clone();
                    let goal_clone_for_edit = goal.clone();
                    html! {
                        <GoalCard
                            id={goal.id}
                            name={goal.name.clone()}
                            description={goal.description.clone()}
                            category={goal.category.clone()}
                            status={goal.status.clone()}
                            goal_type={goal.goal_type.clone()}
                            date_start={goal.date_start.clone()}
                            date_end={goal.date_end.clone()}
                            days_remaining={goal.days_remaining}
                            progress_percentage={goal.progress_percentage}
                            on_goal_delete={on_delete_goal.clone()}
                            on_goal_updated={Callback::from(move |_| reload_goals.emit(()))}
                            on_edit={Callback::from(move |_| on_edit_goal.emit(goal_clone_for_edit.clone()))}
                            on_status_change={on_goal_status_update.clone()}
                        />
                    }
                })
            }
            </div>

            if *show_form {
                <GoalForm
                    visible={true}
                    goal_to_edit={(*goal_to_edit).clone()}
                    on_close={on_close_form}
                    on_save={on_save_goal}
                />
            }
        </div>
    }
}