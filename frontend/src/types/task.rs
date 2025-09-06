use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TaskDuration {
    MeiaHora,
    UmaHora,
    Manha,
    Tarde,
    Noite,
}

impl TaskDuration {
    pub fn all() -> Vec<TaskDuration> {
        vec![
            TaskDuration::MeiaHora,
            TaskDuration::UmaHora,
            TaskDuration::Manha,
            TaskDuration::Tarde,
            TaskDuration::Noite,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            TaskDuration::MeiaHora => "Meia Hora",
            TaskDuration::UmaHora => "Uma Hora",
            TaskDuration::Manha => "Manhã",
            TaskDuration::Tarde => "Tarde",
            TaskDuration::Noite => "Noite",
        }
    }

    pub fn value(&self) -> &'static str {
        match self {
            TaskDuration::MeiaHora => "MeiaHora",
            TaskDuration::UmaHora => "UmaHora",
            TaskDuration::Manha => "Manhã",
            TaskDuration::Tarde => "Tarde",
            TaskDuration::Noite => "Noite",
        }
    }

    pub fn from_value(value: &str) -> Option<TaskDuration> {
        match value {
            // to do duas horas e madrugada
            "MeiaHora" => Some(TaskDuration::MeiaHora),
            "UmaHora" => Some(TaskDuration::UmaHora),
            "Manhã" => Some(TaskDuration::Manha),
            "Tarde" => Some(TaskDuration::Tarde),
            "Noite" => Some(TaskDuration::Noite),
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
    pub id: Option<u32>,
    pub title: String,
    pub category: String,
    pub description: String,
    pub duration: TaskDuration,
    pub begin_date: String, // ISO 8601 format: YYYY-MM-DDTHH:MM
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Task {
    pub fn new(
        title: String,
        category: String,
        description: String,
        duration: TaskDuration,
        begin_date: String,
    ) -> Self {
        Self {
            id: None,
            title,
            category,
            description,
            duration,
            begin_date,
            created_at: None,
            updated_at: None,
        }
    }
}
