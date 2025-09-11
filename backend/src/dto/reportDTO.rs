use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, serde::Serialize)]
pub struct TaskStatsYearResponse {
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub percentage: f64,
    pub year: i32,
    pub most_productive_shift: String,
    pub most_used_category: String,
    pub most_productive_month: String,
    pub most_productive_week: String,
}

#[derive(Debug, serde::Serialize)]
pub struct TaskStatsMonthResponse {
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub percentage: f64,
    pub year: i32,
    pub month: i32,
    pub most_productive_shift: String,
    pub most_used_category: String,
    pub most_productive_week: String,
}

#[derive(Debug, serde::Serialize)]
pub struct TaskStatsWeekResponse {
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub percentage: f64,
    pub year: i32,
    pub week: i32,
    pub most_productive_shift: String,
    pub most_used_category: String,
    pub most_productive_day: String,
}
