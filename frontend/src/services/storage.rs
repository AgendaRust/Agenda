use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct UserPreferences {
    pub dark_mode: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self { dark_mode: false }
    }
}
