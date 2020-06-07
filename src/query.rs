use serde_json::json;

pub fn query(req: &str) -> String {
    if !req.is_empty() {
        return String::new();
    }
    let response = json!({
        "provider": std::env::var("IDENTIFIER").expect("Failed to obtain IDENTIFIER from env"),
        "services": [
            {
                "id": "AppleScript",
                "title": env!("PROVIDER_NAME"),
                "subtitle": env!("PROVIDER_DESC")
            }
        ]
    });
    serde_json::to_string(&response).expect("Failed to serialize Value")
}
