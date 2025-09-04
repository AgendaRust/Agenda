pub mod auth;
pub mod storage;
pub use storage::*;
pub mod tasks;

pub const API_URL: &str = "http://127.0.0.1:8000";