#[cfg(feature = "json")]
use crate::Message;

#[cfg(feature = "json")]
pub fn messages_to_json(messages: &[Message]) -> String {
    serde_json::to_string(messages).unwrap()
}

#[cfg(feature = "json")]
pub fn json_to_messages(json_str: &str) -> Vec<Message> {
    serde_json::from_str(json_str).unwrap()
}

#[cfg(all(test, feature = "json"))]
mod tests {
    use super::*;

    #[test]
    fn test_messages_to_json_and_back() {
        // Beispiel-Daten
        let messages = vec![
            Message {
                date: "2025-11-02".into(),
                datetime: "2025-11-02T12:34:56".into(),
                sender: "Alice".into(),
                message: "Hallo Welt!".into(),
            },
            Message {
                date: "2025-11-03".into(),
                datetime: "2025-11-03T08:00:00".into(),
                sender: "Bob".into(),
                message: "Guten Morgen!".into(),
            },
        ];

        // In JSON umwandeln
        let json = messages_to_json(&messages);
        assert!(json.contains("Alice"));
        assert!(json.contains("Bob"));

        // Wieder zurück konvertieren
        let parsed = json_to_messages(&json);

        // Vergleiche, ob gleich
        assert_eq!(messages, parsed);
    }

    #[test]
    fn test_empty_messages_to_json() {
        let empty: Vec<Message> = vec![];
        let json = messages_to_json(&empty);
        assert_eq!(json, "[]");

        let parsed = json_to_messages(&json);
        assert!(parsed.is_empty());
    }
}
