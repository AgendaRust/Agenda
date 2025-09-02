use sea_orm::prelude::DateTimeUtc;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaskDto {
    pub title: String,
    pub description: String,
    pub status: String,
    pub begin_date: DateTimeUtc,
    pub complete_date: DateTimeUtc,
    pub category: String,
    pub r#type: String,
}
