pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace("_", "")
        .split_terminator(&[' ', '-'][..])
        .flat_map(|word| {
            if !word.is_empty() && word.chars().all(char::is_uppercase) {
                vec![
                    word.chars()
                        .next()
                        .expect("Failed to get the first letter")
                        .to_string(),
                ]
            } else {
                word.char_indices()
                    .filter(|&(i, c)| i == 0 || (c.is_uppercase()))
                    .map(|(_, c)| c.to_uppercase().to_string())
                    .collect::<Vec<String>>()
            }
        })
        .collect::<String>()
}
