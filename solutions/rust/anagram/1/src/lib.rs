use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_map = get_map(word);

    possible_anagrams
        .iter()
        .filter(|&candidate| {
            let candidate_map = get_map(candidate);
            word_map == candidate_map && word.to_lowercase() != candidate.to_lowercase()
        })
        .copied()
        .collect()
}

pub fn get_map(word: &str) -> HashMap<char, u32> {
    let mut word_map = HashMap::new();

    for char in word.to_lowercase().chars() {
        *word_map
            .entry(char)
            .or_insert(0) += 1;
    }

    word_map
}