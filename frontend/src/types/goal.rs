#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Goal {
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
}