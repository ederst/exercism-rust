use itertools::Itertools;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let result = possible_anagrams
        .iter()
        .cloned()
        .filter(|possible_anagram| {
            word.chars().sorted().collect::<String>()
                == possible_anagram.chars().sorted().collect::<String>()
        })
        .collect();

    println!("{:?}", result);

    //let result = HashSet::from_iter(possible_anagrams.iter().cloned());

    result
}

pub fn main() {
    let word = "diaper";

    let inputs = ["hello", "world", "zombies", "pants"];

    // {:?} -> debut output
    println!("{:?}", anagrams_for(word, &inputs));
}
