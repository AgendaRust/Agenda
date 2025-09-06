use yew::{function_component, html, Html, Properties, use_state, Callback, MouseEvent};
use crate::types::TaskDuration;

fn format_time_display(time: &str, duration: &TaskDuration) -> String {
    match duration {
        TaskDuration::Manha => "Manhã".to_string(),
        TaskDuration::Tarde => "Tarde".to_string(),
        TaskDuration::Noite => "Noite".to_string(),
        TaskDuration::MeiaHora | TaskDuration::UmaHora => {
            // Parse the time (format: "HH:MM")
            if let Some((hour_str, minute_str)) = time.split_once(':') {
                if let (Ok(hour), Ok(minute)) = (hour_str.parse::<u32>(), minute_str.parse::<u32>()) {
                    let start_time = format!("{:02}:{:02}", hour, minute);
                    
                    let duration_minutes = match duration {
                        TaskDuration::MeiaHora => 30,
                        TaskDuration::UmaHora => 60,
                        _ => 0, // This shouldn't happen given the outer match
                    };
                    
                    let total_minutes = hour * 60 + minute + duration_minutes;
                    let end_hour = (total_minutes / 60) % 24; // Handle day overflow
                    let end_minute = total_minutes % 60;
                    let end_time = format!("{:02}:{:02}", end_hour, end_minute);
                    
                    format!("{} - {}", start_time, end_time)
                } else {
                    // Fallback if parsing fails
                    format!("Time: {}", time)
                }
            } else {
                // Fallback if time format is unexpected
                format!("Time: {}", time)
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TaskCardProps {
    pub title: String,
    pub category: String,
    pub description: String,
    pub date: String,
    pub time: String,
    pub duration: TaskDuration,
}

#[function_component(TaskCard)]
pub fn task_card(props: &TaskCardProps) -> Html {
    let show_info = use_state(|| false);

    let toggle_info = {
        let show_info = show_info.clone();
        Callback::from(move |_: MouseEvent| {
            show_info.set(!*show_info);
        })
    };

    html! {
        <div class="task-card" onclick={toggle_info}>
            <div class="task-header">
                <h3 class="task-title">{ &props.title }</h3>
                if *show_info {
                    <div class="task-actions">
                        <button class="edit-button">{ "Edit" }</button>
                        <button class="delete-button">{ "Delete" }</button>
                    </div>
                }
            </div>
            <div class="task-body">
                if *show_info {
                    <p class="task-description">{ &props.description }</p>
                }
                <div class="task-datetime">
                    <span class="task-date">{ format!("Due Date: {}", &props.date) }</span>
                    if *show_info {
                        <span class="task-time">{ format_time_display(&props.time, &props.duration) }</span>
                    }
                </div>
            </div>
        </div>
    }
}