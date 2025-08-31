/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(' ', "");

    if code == "0" {
        return false;
    }

    for c in code.chars() {
        if !"0123456789".contains(c) {
            return false;
        }
    }

    matches!(
        code.chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .map(|(i, j)| {
                if i % 2 != 0 {
                    let value = j * 2;

                    if value > 9 { value - 9 } else { value }
                } else {
                    j
                }
            })
            .sum::<u32>()
            % 10,
        0
    )
}
