use serde_json::Value;

pub fn parse_error(error: &str) -> String {
    match serde_json::from_str::<Value>(error) {
        Ok(json) => {
            if let Some(message) = json["error"]["message"].as_str() {
                return message.to_string();
            }
        },
        Err(_) => {},
    }
    error.to_string()
}