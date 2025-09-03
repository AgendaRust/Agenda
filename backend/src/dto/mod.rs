pub mod authDTO;
pub mod taskDTO;
pub mod reminder_dto;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNote {
    pub text: String,
    pub bolsonar: String
}
