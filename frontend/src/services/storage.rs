use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const THEME_KEY: &str = "agenda_theme";

#[derive(Serialize, Deserialize)]
pub struct UserPreferences {
    pub dark_mode: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self { dark_mode: false }
    }
}

pub struct StorageService;

impl StorageService {
    pub fn get_theme() -> bool {
        LocalStorage::get::<UserPreferences>(THEME_KEY)
            .unwrap_or_default()
            .dark_mode
    }

    pub fn set_theme(dark_mode: bool) {
        let preferences = UserPreferences { dark_mode };
        let _ = LocalStorage::set(THEME_KEY, preferences);
    }
}
