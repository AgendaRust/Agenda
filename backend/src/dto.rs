use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNote {
    pub text: String,
}
