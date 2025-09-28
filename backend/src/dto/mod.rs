pub mod auth_dto;
pub mod task_dto;

pub mod reminder_dto;
pub mod goal_dto;
pub mod task_update_dto;
pub mod report_dto;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNote {
    pub text: String,
    pub bolsonar: String
}
