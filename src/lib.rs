use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub date: String,
    pub datetime: String,
    pub sender: String,
    pub message: String,
}

/// Parst einen WhatsApp-Chatverlauf (z. B. exportierte .txt-Datei)
/// und gibt einen Vektor von Nachrichten zurück.
pub fn parsetxt(content: String) -> Vec<Message> {
    let re = Regex::new(r"^(\d{2}\.\d{2}\.\d{2}), (\d{2}:\d{2}) - (.*)").unwrap();
    let mut messages = Vec::new();
    let mut current_message: Option<Message> = None;

    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            // Vorherige Nachricht speichern
            if let Some(msg) = current_message.take() {
                messages.push(msg);
            }

            let date = caps[1].to_string();
            let time = caps[2].to_string();
            let rest = caps[3].to_string();

            if let Some((sender, msg)) = rest.split_once(": ") {
                current_message = Some(Message {
                    date: date.clone(),
                    datetime: format!("{}, {}", date, time),
                    sender: sender.to_string(),
                    message: msg.to_string(),
                });
            } else {
                current_message = Some(Message {
                    date: date.clone(),
                    datetime: format!("{}, {}", date, time),
                    sender: String::new(),
                    message: rest.to_string(),
                });
            }
        } else if let Some(ref mut msg) = current_message {
            // Mehrzeilige Nachricht fortsetzen
            msg.message.push('\n');
            msg.message.push_str(line);
        }
    }

    // Letzte Nachricht hinzufügen
    if let Some(msg) = current_message {
        messages.push(msg);
    }

    messages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_message() {
        let input = "08.05.24, 12:14 - sammy: Wer ist eig dein Partner?".to_string();
        let result = parsetxt(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].sender, "sammy");
        assert_eq!(result[0].message, "Wer ist eig dein Partner?");
    }

    #[test]
    fn test_two_messages() {
        let input = "\
08.05.24, 12:14 - sammy: Wer ist eig dein Partner?
08.05.24, 13:19 - Ole: Cameron"
            .to_string();

        let result = parsetxt(input);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].sender, "sammy");
        assert_eq!(result[0].message, "Wer ist eig dein Partner?");
        assert_eq!(result[1].sender, "Ole");
        assert_eq!(result[1].message, "Cameron");
    }

    #[test]
    fn test_system_message() {
        let input = "\
11.10.23, 15:44 - Nachrichten und Anrufe sind Ende-zu-Ende-verschlüsselt.
Mehr erfahren"
            .to_string();

        let result = parsetxt(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].sender, "");
        assert!(result[0].message.contains("Ende-zu-Ende-verschlüsselt"));
        assert!(result[0].message.contains("Mehr erfahren"));
    }

    #[test]
    fn test_multiline_message() {
        let input = "\
26.03.25, 19:02 - sammy: Das ist die erste Zeile
und das ist die zweite Zeile
26.03.25, 19:06 - Ole: Alles klar"
            .to_string();

        let result = parsetxt(input);
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0].message,
            "Das ist die erste Zeile\nund das ist die zweite Zeile"
        );
    }

    #[test]
    fn test_message_without_sender() {
        let input =
            "23.11.23, 22:20 - Nachrichten und Anrufe sind Ende-zu-Ende-verschlüsselt.".to_string();

        let result = parsetxt(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].sender, "");
    }

    #[test]
    fn test_last_message_included() {
        let input = "\
08.05.24, 12:14 - sammy: Erste Nachricht
08.05.24, 13:19 - Ole: Zweite Nachricht"
            .to_string();

        let result = parsetxt(input);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].sender, "sammy");
        assert_eq!(result[1].sender, "Ole");
    }

    #[test]
    fn test_mixed_content() {
        let input = "\
11.10.23, 15:44 - Nachrichten und Anrufe sind Ende-zu-Ende-verschlüsselt.
23.11.23, 22:20 - sammy: Hey
23.11.23, 22:21 - Ole: Hi!
23.11.23, 22:22 - Systemmeldung ohne Sender"
            .to_string();

        let result = parsetxt(input);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].sender, "");
        assert_eq!(result[1].sender, "sammy");
        assert_eq!(result[2].sender, "Ole");
        assert_eq!(result[3].sender, "");
    }
}
