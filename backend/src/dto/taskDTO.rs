use sea_orm::prelude::DateTimeUtc;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaskDto {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
    pub description: String,
    pub status: String,
    pub begin_date: DateTimeUtc,
    pub complete_date: DateTimeUtc,
    pub category: String,
    pub r#type: String,
}
