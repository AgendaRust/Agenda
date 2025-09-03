use sea_orm::prelude::DateTimeUtc;
use validator::Validate;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Validate)]
pub struct TaskDto {
    #[validate(length(min = 3, message = "O título não pode estar vazio."))]
    pub title: String,

    pub description: String,

    #[validate(length(min = 5, message = "O status não pode estar vazio."))]
    pub status: String,

    pub begin_date: DateTimeUtc,

    pub complete_date: DateTimeUtc,

    #[validate(length(min = 5, message = "A categoria não pode estar vazia."))]
    pub category: String,

    #[validate(length(min = 5, message = "O tipo não pode estar vazio."))]
    pub r#type: String,
}