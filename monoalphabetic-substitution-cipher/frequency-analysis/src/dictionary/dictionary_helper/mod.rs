use std::collections::HashMap;

use regex::Regex;

use super::super::helper;

pub fn calculate_word_frequeny_with_length(sentence: &str, length: usize) -> HashMap<char, u32> {
    let words = get_all_words(sentence);
    let mut one_letter_words_frequeny: HashMap<char, u32> = HashMap::new();

    for word in &words {
        if word.len() == length {
            let counter = one_letter_words_frequeny
                .entry(word.chars().next().unwrap())
                .or_insert(0);
            *counter += 1;
        }
    }

    for (key, value) in one_letter_words_frequeny.iter() {
        trace!("Word '{}' was counted {} times.", key, value);
    }

    one_letter_words_frequeny
}

fn get_all_words(sentence: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut sentence: String = replace_all_non_alphabet(sentence);
    sentence = remove_all_extra_spaces(sentence);

    for word in sentence.split(' ') {
        words.push(String::from(word));
    }

    words
}

fn replace_all_non_alphabet(replacing: &str) -> String {
    let mut returning = String::from(replacing);

    for (_index, character) in replacing.chars().enumerate() {
        if !helper::ALPHABET.contains(&character) && character != ' ' {
            returning = returning.replace(character, "");
        }
    }

    returning
}

fn remove_all_extra_spaces(replacing: String) -> String {
    let returning = replacing.trim();
    let regex = Regex::new(r"\s+").unwrap();
    return regex.replace_all(returning, " ").into_owned();
}

#[cfg(test)]
mod tests;
