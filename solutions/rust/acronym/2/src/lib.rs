pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_terminator(&[' ', '-', '_'][..])
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .map(|c| c.to_uppercase().to_string())
        .collect()
}
