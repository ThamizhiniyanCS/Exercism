/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), c| {
            c.to_digit(10).map(|mut digit| {
                if count % 2 != 0 {
                    digit *= 2;
                    if digit > 9 {
                        digit -= 9;
                    }
                }
                (sum + digit, count + 1)
            })
        })
        .is_some_and(|(sum, count)| sum % 10 == 0 && count > 1)
}
