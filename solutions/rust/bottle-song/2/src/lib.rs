static NUM_STRING_ARRAY: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

fn capitalize(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn verse(start_bottles: u32, take_down: u32) -> String {
    format!(
        "{} green bottle{} hanging on the wall,\n",
        capitalize(NUM_STRING_ARRAY[start_bottles as usize]),
        if start_bottles == 1 { "" } else { "s" }
    ) + format!(
        "{} green bottle{} hanging on the wall,\n",
        capitalize(NUM_STRING_ARRAY[start_bottles as usize]),
        if start_bottles == 1 { "" } else { "s" }
    )
    .as_str()
        + "And if one green bottle should accidentally fall,\n"
        + format!(
            "There'll be {} green bottle{} hanging on the wall.{}",
            NUM_STRING_ARRAY[(start_bottles - 1) as usize],
            if start_bottles - 1 == 1 { "" } else { "s" },
            if take_down == 1 { "" } else { "\n" },
        )
        .as_str()
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| {
            verse(
                if start_bottles > 1 {
                    start_bottles - i
                } else {
                    start_bottles
                },
                take_down,
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
