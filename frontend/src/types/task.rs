use std::fmt;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskDuration {
    MeiaHora,
    UmaHora,
    DuasHoras,
    Manha,
    Tarde,
    Noite,
    Madrugada,
}

impl TaskDuration {
    pub fn all() -> Vec<TaskDuration> {
        vec![
            TaskDuration::MeiaHora,
            TaskDuration::UmaHora,
            TaskDuration::DuasHoras,
            TaskDuration::Manha,
            TaskDuration::Tarde,
            TaskDuration::Noite,
            TaskDuration::Madrugada,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TaskDuration::MeiaHora => "Meia Hora",
            TaskDuration::UmaHora => "Uma Hora",
            TaskDuration::DuasHoras => "Duas Horas",
            TaskDuration::Manha => "Manhã",
            TaskDuration::Tarde => "Tarde",
            TaskDuration::Noite => "Noite",
            TaskDuration::Madrugada => "Madrugada",
        }
    }

    pub fn value(&self) -> &'static str {
        match self {
            TaskDuration::MeiaHora => "MeiaHora",
            TaskDuration::UmaHora => "UmaHora",
            TaskDuration::DuasHoras => "DuasHoras",
            TaskDuration::Manha => "Manha",
            TaskDuration::Tarde => "Tarde",
            TaskDuration::Noite => "Noite",
            TaskDuration::Madrugada => "Madrugada",
        }
    }

    pub fn from_value(value: &str) -> Option<TaskDuration> {
        match value {
            "MeiaHora" => Some(TaskDuration::MeiaHora),
            "UmaHora" => Some(TaskDuration::UmaHora),
            "DuasHoras" => Some(TaskDuration::DuasHoras),   
            "Manhã" | "Manha" => Some(TaskDuration::Manha),
            "Tarde" => Some(TaskDuration::Tarde),
            "Noite" => Some(TaskDuration::Noite),
            "Madrugada" => Some(TaskDuration::Madrugada),
            _ => None,
        }
    }
}

impl Default for TaskDuration {
    fn default() -> Self {
        TaskDuration::MeiaHora
    }
}

impl fmt::Display for TaskDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub user_id: u32,
    pub description: String,
    pub status: String,
    pub begin_date: DateTime<chrono::Utc>,
    pub complete_date: DateTime<chrono::Utc>,
    pub category: String,
    #[serde(rename = "type")]
    pub task_type: String,
}

impl Task {
    pub fn new(
        id: u32,
        title: String,
        user_id: u32,
        description: String,
        status: String,
        begin_date: DateTime<chrono::Utc>,
        complete_date: DateTime<chrono::Utc>,
        category: String,
        task_type: String,
    ) -> Self {
        Self {
            id,
            title,
            user_id,
            description,
            status,
            begin_date,
            complete_date,
            category,
            task_type,
        }
    }
}
