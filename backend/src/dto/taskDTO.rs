use sea_orm::prelude::DateTimeUtc;
use validator::Validate;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Validate)]
pub struct TaskDto {
    #[validate(length(min = 3, message = "O título não pode estar vazio."))]
    pub title: String,

    #[validate(length(min = 5, message = "A categoria não pode estar vazia."))]
    pub category: String,

    pub description: String,
    pub begin_date: DateTimeUtc,
    pub r#type: String,
}