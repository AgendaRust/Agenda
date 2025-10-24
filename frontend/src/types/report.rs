use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatsWeekResponse {
    pub year: i32,
    pub week: i32,
    pub total_tasks: i64,
    pub executed_tasks: i64,
    pub pendent_tasks: i64,
    pub delayed_tasks: i64,
    pub percentage_tasks: f64,
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

// Implementações default para casos de erro
impl Default for StatsYearResponse {
    fn default() -> Self {
        Self {
            year: 0,
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

impl Default for StatsMonthResponse {
    fn default() -> Self {
        Self {
            year: 0,
            month: 0,
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

impl Default for StatsWeekResponse {
    fn default() -> Self {
        Self {
            year: 0,
            week: 0,
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
