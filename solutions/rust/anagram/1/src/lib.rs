use std::collections::{HashMap, HashSet};

fn generate_word_dict(word: &str) -> HashMap<char, u8> {
    let mut word_dict: HashMap<char, u8> = HashMap::new();

    for character in word.chars() {
        word_dict
            .entry(character)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    word_dict
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word: String = word.to_lowercase();
    let word_dict: HashMap<char, u8> = generate_word_dict(&word);
    let mut valid_anagrams: HashSet<&'a str> = HashSet::new();

    for anagram in possible_anagrams {
        let a = anagram.to_lowercase();

        if word == a {
            continue;
        }

        let anagram_dict: HashMap<char, u8> = generate_word_dict(a.as_ref());

        if word_dict == anagram_dict {
            valid_anagrams.insert(anagram.as_ref());
        }
    }

    valid_anagrams
}
