use std::collections::HashSet;

fn sort_word(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect::<Vec<char>>();

    chars.sort_unstable(); // Faster than sort()
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let sorted_word = sort_word(word);

    possible_anagrams
        .iter()
        .filter(|anagram| {
            word.to_lowercase() != anagram.to_lowercase() && sort_word(anagram) == sorted_word
        })
        .copied()
        .collect()
}
