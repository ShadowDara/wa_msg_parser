use crate Message;

#[cfg(feature = "json")]
pub fn messages_to_json(messages: &[Message]) -> String {
    serde_json::to_string(messages).unwrap()
}

#[cfg(feature = "json")]
pub fn json_to_messages(json_str: &str) -> Vec<Message> {
    serde_json::from_str(json_str).unwrap()
}
