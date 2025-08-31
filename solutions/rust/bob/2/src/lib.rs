pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = message.ends_with("?");
    let is_yelling =
        message.chars().any(|c| c.is_alphabetic()) && message == message.to_uppercase();

    match (is_questioning, is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (false, true) => "Whoa, chill out!",
        (true, false) => "Sure.",
        _ => "Whatever.",
    }
}
