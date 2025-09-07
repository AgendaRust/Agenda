use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub enum ReportType {
    Weekly,
    Monthly,
    Annual,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportRequestDto {
    pub report_type: ReportType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportResponseDto {
    pub period: String,
    pub report_type: ReportType,
    pub goal_summary: GoalSummary,
    pub task_summary: TaskSummary,
    pub productivity_insights: ProductivityInsights,
    pub category_analysis: CategoryAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalSummary {
    pub total_goals: i32,
    pub completed_goals: i32,
    pub partially_completed_goals: i32,
    pub not_completed_goals: i32,
    pub completion_percentage: f32,
    pub partial_completion_percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSummary {
    pub total_tasks: i32,
    pub completed_tasks: i32,
    pub partially_completed_tasks: i32,
    pub postponed_tasks: i32,
    pub completion_percentage: f32,
    pub partial_completion_percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductivityInsights {
    pub most_productive_periods: Vec<ProductivePeriod>,
    pub most_productive_time_slots: Vec<TimeSlotProductivity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductivePeriod {
    pub period: String,
    pub completed_tasks: i32,
    pub completed_goals: i32,
    pub total_score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSlotProductivity {
    pub time_slot: String, // "morning", "afternoon", "evening"
    pub completed_tasks: i32,
    pub total_tasks: i32,
    pub productivity_percentage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryAnalysis {
    pub top_task_categories: Vec<CategoryStats>,
    pub top_goal_categories: Vec<CategoryStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryStats {
    pub category_name: String,
    pub total_items: i32,
    pub completed_items: i32,
    pub completion_percentage: f32,
}