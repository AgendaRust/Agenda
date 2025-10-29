use yew::{function_component, html, Callback, Html, MouseEvent, Properties, classes};

use crate::types::goal::Goal;
#[derive(Properties, PartialEq, Clone)]
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
    pub on_edit: Callback<Goal>,
    pub on_status_change: Callback<(i32, String)>,
}


#[function_component(GoalCard)]
    pub fn goal_card(props: &GoalCardProps) -> Html {
        let on_delete_click = {
            let on_goal_delete = props.on_goal_delete.clone();
            let goal_id = props.id;
            let goal_name = props.name.clone();
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                e.stop_propagation();
                if web_sys::window().unwrap().confirm_with_message(&format!("Tem certeza que deseja excluir a meta '{}'?", goal_name)).unwrap() {
                    on_goal_delete.emit(goal_id);
                }
            })
        };

        let on_edit_click = {
            let on_edit = props.on_edit.clone();
            let goal_data = Goal {
                id: props.id,
                name: props.name.clone(),
                description: props.description.clone(),
                category: props.category.clone(),
                status: props.status.clone(),
                goal_type: props.goal_type.clone(),
                date_start: props.date_start.clone(),
                date_end: props.date_end.clone(),
                days_remaining: props.days_remaining,
                progress_percentage: props.progress_percentage,
            };
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                e.stop_propagation();
                on_edit.emit(goal_data.clone());
            })
        };

        let on_complete_click = {
            let on_status_change = props.on_status_change.clone();
            let goal_id = props.id;
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                e.stop_propagation();
                on_status_change.emit((goal_id, "Concluída".to_string()));
            })
        };

        let on_cancel_click = {
            let on_status_change = props.on_status_change.clone();
            let goal_id = props.id;
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                e.stop_propagation();
                on_status_change.emit((goal_id, "Cancelada".to_string()));
            })
        };

        let status_class = props.status.to_lowercase().replace('_', "-");
        let goal_type_class = props.goal_type.to_lowercase();
        
        let goal_type_label = match props.goal_type.as_str() {
            "weekly" => "Semanal",
            "monthly" => "Mensal",
            "annual" => "Anual",
            _ => &props.goal_type,
        };

    html! {
        <div class="goal-card">
            <div class="goal-header">
                <h3 class="goal-title">{ &props.name }</h3>
                <div class="goal-actions">
                    <button class="edit-button" onclick={on_edit_click}>{ "Editar" }</button>
                    <button class="delete-button" onclick={on_delete_click}>{ "Excluir" }</button>
                </div>
            </div>
            <div class="goal-body">
                <div class="goal-meta">
                    <span class={classes!("goal-type", goal_type_class)}>{ goal_type_label }</span>
                    { props.category.as_ref().map_or(html!{}, |cat| html!{ <span class="goal-category">{cat}</span> }) }
                </div>

                { props.description.as_ref().filter(|d| !d.is_empty()).map_or_else(
                    || html!{ <p class="goal-description empty">{ "Sem descrição" }</p> },
                    |desc| html!{ <p class="goal-description">{desc}</p> }
                )}

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
                        <div class="progress-fill" style={format!("width: {}%", props.progress_percentage.min(100.0))} />
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
                    <span class={classes!("status-indicator", status_class.clone())}>{ &props.status }</span>
                    { if props.status == "Em andamento" || props.status == "NotStarted" {
                        html! {
                            <div class="quick-actions">
                                <button class="edit-button" onclick={on_complete_click}>{ "✓ Concluir" }</button>
                                <button class="delete-button" onclick={on_cancel_click}>{ "✗ Cancelar" }</button>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>
        </div>
    }
}