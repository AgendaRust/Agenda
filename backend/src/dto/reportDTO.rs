use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, serde::Serialize)]
pub struct StatsYearResponse {
    pub year: i32,
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub pendent_tasks: i64,
    pub delayed_tasks: i64,
    pub percentage_tasks: f64,
    pub most_productive_shift_tasks: String,
    pub most_used_category_tasks: String,
    pub most_productive_month_tasks: String,
    pub most_productive_week_tasks: String,
    pub classification_tasks: String,
    pub total_goals: i64,
    pub pendent_goals: i64,
    pub executed_goals: i64,
    pub delayed_goals: i64,
    pub percentage_goals: f64,
    pub most_productive_shift_goals: String,
    pub most_used_category_goals: String,
    pub most_productive_month_goals: String,
    pub most_productive_week_goals: String,
    pub classification_goals: String,
}

#[derive(Debug, serde::Serialize)]
pub struct StatsMonthResponse {
    pub year: i32,
    pub month: i32,
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub pendent_tasks: i64,
    pub delayed_tasks: i64,
    pub percentage_tasks: f64,
    pub most_productive_shift_tasks: String,
    pub most_used_category_tasks: String,
    pub most_productive_week_tasks: String,
    pub classification_tasks: String,
    pub total_goals: i64,
    pub pendent_goals: i64,
    pub executed_goals: i64,
    pub delayed_goals: i64,
    pub percentage_goals: f64,
    pub most_productive_shift_goals: String,
    pub most_used_category_goals: String,
    pub most_productive_week_goals: String,
    pub classification_goals: String,
}

#[derive(Debug, serde::Serialize)]
pub struct StatsWeekResponse {
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub pendent_tasks: i64,
    pub delayed_tasks: i64,
    pub percentage_tasks: f64,
    pub year: i32,
    pub week: i32,
    pub most_productive_shift_tasks: String,
    pub most_used_category_tasks: String,
    pub most_productive_day_tasks: String,
    pub classification_tasks: String,
    pub total_goals: i64,
    pub pendent_goals: i64,
    pub executed_goals: i64,
    pub delayed_goals: i64,
    pub percentage_goals: f64,
    pub most_productive_shift_goals: String,
    pub most_used_category_goals: String,
    pub most_productive_day_goals: String,
    pub classification_goals: String,
}


impl StatsYearResponse {
    pub fn default_for_year(year: i32) -> Self {
        Self {
            year,
            total_tasks: 0,
            executed_tasks: 0,
            pendent_tasks: 0,
            delayed_tasks: 0,
            percentage_tasks: 0.0,
            most_productive_shift_tasks: "N/A".to_string(),
            most_used_category_tasks: "N/A".to_string(),
            most_productive_month_tasks: "N/A".to_string(),
            most_productive_week_tasks: "N/A".to_string(),
            classification_tasks: "N/A".to_string(),
            total_goals: 0,
            executed_goals: 0,
            pendent_goals: 0,
            delayed_goals: 0,
            percentage_goals: 0.0,
            most_productive_shift_goals: "N/A".to_string(),
            most_used_category_goals: "N/A".to_string(),
            most_productive_month_goals: "N/A".to_string(),
            most_productive_week_goals: "N/A".to_string(),
            classification_goals: "N/A".to_string(),
        }
    }
}

impl StatsMonthResponse {
    pub fn default_for_month(year: i32, month: i32) -> Self {
        Self {
            year,
            month,
            total_tasks: 0,
            executed_tasks: 0,
            pendent_tasks: 0,
            delayed_tasks: 0,
            percentage_tasks: 0.0,
            most_productive_shift_tasks: "N/A".to_string(),
            most_used_category_tasks: "N/A".to_string(),
            most_productive_week_tasks: "N/A".to_string(),
            classification_tasks: "N/A".to_string(),
            total_goals: 0,
            executed_goals: 0,
            pendent_goals: 0,
            delayed_goals: 0,
            percentage_goals: 0.0,
            most_productive_shift_goals: "N/A".to_string(),
            most_used_category_goals: "N/A".to_string(),
            most_productive_week_goals: "N/A".to_string(),
            classification_goals: "N/A".to_string(),
        }
    }
}

impl StatsWeekResponse {
    pub fn default_for_week(year: i32, week: i32) -> Self {
        Self {
            year,
            week,
            total_tasks: 0,
            executed_tasks: 0,
            pendent_tasks: 0,
            delayed_tasks: 0,
            percentage_tasks: 0.0,
            most_productive_shift_tasks: "N/A".to_string(),
            most_used_category_tasks: "N/A".to_string(),
            most_productive_day_tasks: "N/A".to_string(),
            classification_tasks: "N/A".to_string(),
            total_goals: 0,
            executed_goals: 0,
            pendent_goals: 0,
            delayed_goals: 0,
            percentage_goals: 0.0,
            most_productive_shift_goals: "N/A".to_string(),
            most_used_category_goals: "N/A".to_string(),
            most_productive_day_goals: "N/A".to_string(),
            classification_goals: "N/A".to_string(),
        }
    }
}