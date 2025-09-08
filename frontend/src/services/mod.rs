pub mod auth;
pub mod storage;
pub use storage::*;
pub mod tasks;
pub mod reminder_service;
pub const API_URL: &str = "http://127.0.0.1:8000";