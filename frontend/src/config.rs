pub fn get_api_url() -> &'static str {
    // When using trunk serve (development), frontend runs on :8080 and backend on :8000
    // When using docker or serving from backend (production), use relative path
    if cfg!(debug_assertions) {
        // Development mode - trunk serve
        "http://localhost:8000/api"
    } else {
        // Production mode - served by backend
        "/api"
    }
}
