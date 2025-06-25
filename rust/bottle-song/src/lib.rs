const NUM_STRING_ARRAY: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

fn capitalize(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn bottle_word(n: u32) -> (&'static str, &'static str) {
    let word = NUM_STRING_ARRAY[n as usize];
    let plural = if n == 1 { "" } else { "s" };
    (word, plural)
}

fn verse(start_bottles: u32, is_last: bool) -> String {
    let (start_word, start_plural) = bottle_word(start_bottles);
    let (next_word, next_plural) = bottle_word(start_bottles - 1);

    format!(
        "{0} green bottle{1} hanging on the wall,\n\
         {0} green bottle{1} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {2} green bottle{3} hanging on the wall.{4}",
        capitalize(start_word),
        start_plural,
        next_word,
        next_plural,
        if is_last { "" } else { "\n" }
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| verse(start_bottles - i, i == take_down - 1))
        .collect::<Vec<String>>()
        .join("\n")
}
