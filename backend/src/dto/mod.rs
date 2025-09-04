pub mod authDTO;
pub mod taskDTO;

pub mod reminder_dto;
pub mod goalDTO;
pub mod taskStatusDTO;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNote {
    pub text: String,
    pub bolsonar: String
}
