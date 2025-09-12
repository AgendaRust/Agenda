use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, serde::Serialize)]
pub struct TaskStatsYearResponse {
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub pendent_tasks: i64,
    pub delayed_tasks: i64,
    pub percentage: f64,
    pub year: i32,
    pub most_productive_shift: String,
    pub most_used_category: String,
    pub most_productive_month: String,
    pub most_productive_week: String,
    pub classification: String,
}

#[derive(Debug, serde::Serialize)]
pub struct TaskStatsMonthResponse {
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub pendent_tasks: i64,
    pub delayed_tasks: i64,
    pub percentage: f64,
    pub year: i32,
    pub month: i32,
    pub most_productive_shift: String,
    pub most_used_category: String,
    pub most_productive_week: String,
    pub classification: String
}

#[derive(Debug, serde::Serialize)]
pub struct TaskStatsWeekResponse {
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub pendent_tasks: i64,
    pub delayed_tasks: i64,
    pub percentage: f64,
    pub year: i32,
    pub week: i32,
    pub most_productive_shift: String,
    pub most_used_category: String,
    pub most_productive_day: String,
    pub classification: String
}


impl TaskStatsYearResponse {
    pub fn default_for_year(year: i32) -> Self {
        Self {
            total_tasks: 0,
            executed_tasks: 0,
            pendent_tasks: 0,
            delayed_tasks: 0,
            percentage: 0.0,
            year,
            most_productive_shift: "N/A".to_string(),
            most_used_category: "N/A".to_string(),
            most_productive_month: "N/A".to_string(),
            most_productive_week: "N/A".to_string(),
            classification: "N/A".to_string(),
        }
    }
}

impl TaskStatsMonthResponse {
    pub fn default_for_month(year: i32, month: i32) -> Self {
        Self {
            total_tasks: 0,
            executed_tasks: 0,
            pendent_tasks: 0,
            delayed_tasks: 0,
            percentage: 0.0,
            year,
            month,
            most_productive_shift: "N/A".to_string(),
            most_used_category: "N/A".to_string(),
            most_productive_week: "N/A".to_string(),
            classification: "N/A".to_string(),
        }
    }
}

impl TaskStatsWeekResponse {
    pub fn default_for_week(year: i32, week: i32) -> Self {
        Self {
            total_tasks: 0,
            executed_tasks: 0,
            pendent_tasks: 0,
            delayed_tasks: 0,
            percentage: 0.0,
            year,
            week,
            most_productive_shift: "N/A".to_string(),
            most_used_category: "N/A".to_string(),
            most_productive_day: "N/A".to_string(),
            classification: "N/A".to_string(),
        }
    }
}