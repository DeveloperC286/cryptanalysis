use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use super::helper::{ALPHABET};

pub fn one_letter_word_dictionary_corrections(plaintext: String) -> String {
    let mut one_letter_words_frequeny = one_letter_words_frequeny(get_all_words(&plaintext));
    let mut missing_one_letter_words: HashSet<&str> = vec!["a", "i"].into_iter().collect();

    return plaintext;
}

fn one_letter_words_frequeny(words: Vec<String>) -> HashMap<String, u32> {
    let mut one_letter_words_frequeny: HashMap<String, u32> = HashMap::new();

    for word in &words {
        if word.len() == 1 {
            let counter = one_letter_words_frequeny.entry(word.to_string()).or_insert(0);
            *counter += 1;
        }
    }

    return one_letter_words_frequeny;
}

fn get_all_words(sentence: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut sentence: String = replace_all_non_alphabet(sentence);
    sentence = remove_all_extra_spaces(sentence);

    for word in sentence.split(' ') {
        words.push(String::from(word));
    }

    return words;
}

fn replace_all_non_alphabet(replacing: &str) -> String {
    let mut returning = String::from(replacing);

    for (_index, character) in replacing.chars().enumerate() {
        if !ALPHABET.contains(&character) && character != '\'' {
            returning = returning.replace(character, " ");
        }
    }

    return returning;
}

fn remove_all_extra_spaces(replacing: String) -> String {
    let returning = replacing.trim();
    let regex = Regex::new(r"\s+").unwrap();
    return regex.replace_all(returning, " ").into_owned();
}

#[cfg(test)]
mod tests;
