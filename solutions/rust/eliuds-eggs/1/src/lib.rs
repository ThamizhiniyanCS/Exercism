fn decimal_to_binary(value: u32, mut binary_representation: String) -> String {
    if value == 0 {
        return binary_representation.chars().rev().collect();
    }

    let remainder = value % 2;
    binary_representation.push(char::from((remainder as u8) + b'0'));

    decimal_to_binary(value / 2, binary_representation)
}

pub fn egg_count(display_value: u32) -> usize {
    decimal_to_binary(display_value, String::new())
        .chars()
        .filter(|&c| c == '1')
        .count()
}
