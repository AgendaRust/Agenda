pub mod authDTO;
mod taskDTO;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNote {
    pub text: String,
    pub bolsonar: String
}
