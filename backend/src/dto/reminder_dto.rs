use sea_orm::prelude::DateTimeUtc;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct reminder_DTO {
    pub name: String,
    pub category: String,
    pub date_end: DateTimeUtc,
}
