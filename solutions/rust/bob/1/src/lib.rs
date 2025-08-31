fn is_uppercased(message: &str) -> bool {
    for each in message
        .chars()
        .filter(|&c| !c.is_ascii_punctuation() && !c.is_ascii_digit())
    {
        if each != each.to_ascii_uppercase() {
            return false;
        }
    }

    true
}

pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_valid_message = message
        .chars()
        .any(|c| !(c.is_ascii_digit() || c.is_ascii_whitespace() || c.is_ascii_punctuation()));

    if is_valid_message && message.ends_with("?") && is_uppercased(message) {
        return "Calm down, I know what I'm doing!";
    } else if is_valid_message && is_uppercased(message) {
        return "Whoa, chill out!";
    } else if message.ends_with("?") {
        return "Sure.";
    }

    "Whatever."
}
