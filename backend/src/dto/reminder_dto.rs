use sea_orm::prelude::DateTimeUtc;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReminderDto {
    pub name: String,
    pub category: String,
    pub date_end: DateTimeUtc,
}
