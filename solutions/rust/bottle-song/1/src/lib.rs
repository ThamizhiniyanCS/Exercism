use std::collections::HashMap;
use std::sync::LazyLock;

static DICT: LazyLock<HashMap<u32, &str>> = LazyLock::new(|| {
    let mut dict: HashMap<u32, &str> = HashMap::new();
    dict.insert(1, "one");
    dict.insert(2, "two");
    dict.insert(3, "three");
    dict.insert(4, "four");
    dict.insert(5, "five");
    dict.insert(6, "six");
    dict.insert(7, "seven");
    dict.insert(8, "eight");
    dict.insert(9, "nine");
    dict.insert(10, "ten");

    dict
});

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
        capitalize(DICT.get(&start_bottles).unwrap()),
        if start_bottles == 1 { "" } else { "s" }
    ) + format!(
        "{} green bottle{} hanging on the wall,\n",
        capitalize(DICT.get(&start_bottles).unwrap()),
        if start_bottles == 1 { "" } else { "s" }
    )
    .as_str()
        + "And if one green bottle should accidentally fall,\n"
        + format!(
            "There'll be {} green bottle{} hanging on the wall.{}",
            if start_bottles > 1 {
                DICT.get(&(start_bottles - 1)).unwrap()
            } else {
                "no"
            },
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
