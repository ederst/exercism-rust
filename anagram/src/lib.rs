use itertools::Itertools;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word: String = word.to_lowercase();
    let sorted_word: String = lower_word.chars().sorted().collect::<String>();

    possible_anagrams
        .iter()
        .cloned()
        .filter(|possible_anagram| {
            let lower_possible_anagram = possible_anagram.to_lowercase();

            lower_word != lower_possible_anagram
                && sorted_word == lower_possible_anagram.chars().sorted().collect::<String>()
        })
        .collect()
}
