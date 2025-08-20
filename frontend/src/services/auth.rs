use gloo::net::http::Request;

pub fn login() {
    println!("You are logged!");
}

pub async fn get_notes() -> String {
    let url = "http://127.0.0.1:8000";
    let response = Request::get(&url).send().await.unwrap();
    if response.status() == 200 {
        format!("notes resulted {}", response.status_text())
    } else {
        format!("notes not found {}", response.status_text())
    }
}
