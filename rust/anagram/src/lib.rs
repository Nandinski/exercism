use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_size = word.len();
    let word_sorted = sort_word(&word);

    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let candidate = candidate.to_lowercase();
            candidate.len() == word_size
                && word != candidate
                && sort_word(&candidate) == word_sorted
        })
        .copied()
        .collect()
}

fn sort_word(word: &str) -> Vec<char> {
    let mut word_sorted = word.chars().collect::<Vec<char>>();
    word_sorted.sort_unstable();
    word_sorted
}
