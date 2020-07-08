use serde_json::json;

pub fn query(req: &str) {
    if !req.is_empty() {
        return;
    }
    let response = json!({
        "id": "AppleScript",
        "title": env!("PROVIDER_NAME"),
        "subtitle": env!("PROVIDER_DESC")
    });
    let service = serde_json::to_string(&response).expect("Failed to serialize Value");
    println!("{}", service);
}
