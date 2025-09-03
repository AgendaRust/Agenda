pub mod authDTO;
pub mod taskDTO;
pub mod goalDTO;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNote {
    pub text: String,
    pub bolsonar: String
}
