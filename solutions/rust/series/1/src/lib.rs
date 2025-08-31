pub fn series(digits: &str, len: usize) -> Vec<String> {
    let digits_vec: Vec<char> = digits.chars().collect();

    digits_vec
        .windows(len)
        .map(|c| c.iter().collect::<String>())
        .collect()
}
