use super::helper::{get_next_most_frequent, replace_all_occurances, ALPHABET};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

static ONE_LETTER_WORDS: [char; 2] = ['i', 'a'];

pub fn one_letter_word_dictionary_corrections(plaintext: String) -> String {
    let one_letter_words_frequeny = calculate_one_letter_words_frequeny(get_all_words(&plaintext));
    let (missing_one_letter_words, missing_one_letter_words_frequeny) = get_missing_one_letter_words(one_letter_words_frequeny);

    return missing_one_letter_words_corrections(plaintext, missing_one_letter_words, missing_one_letter_words_frequeny);
}

fn missing_one_letter_words_corrections(
    plaintext: String,
    missing_one_letter_words: HashSet<char>,
    missing_one_letter_words_frequeny: HashMap<char, u32>,
) -> String {
    let mut predicted_plaintext = plaintext.chars().collect();

    for missing_one_letter_word in missing_one_letter_words {
        //missing one letter words should be in frequency order.
        let next_most_frequent_word = get_next_most_frequent(&missing_one_letter_words_frequeny);

        predicted_plaintext = replace_all_occurances(&plaintext, predicted_plaintext, next_most_frequent_word, missing_one_letter_word);
        predicted_plaintext = replace_all_occurances(&plaintext, predicted_plaintext, missing_one_letter_word, next_most_frequent_word);
    }

    return predicted_plaintext.into_iter().collect();
}

fn get_missing_one_letter_words(one_letter_words_frequeny: HashMap<char, u32>) -> (HashSet<char>, HashMap<char, u32>) {
    let mut missing_one_letter_words_frequeny = one_letter_words_frequeny.clone();
    let mut missing_one_letter_words: HashSet<char> = HashSet::new();

    for one_letter_word in ONE_LETTER_WORDS.iter() {
        if missing_one_letter_words_frequeny.contains_key(&one_letter_word) {
            missing_one_letter_words_frequeny.remove(&one_letter_word);
        } else {
            missing_one_letter_words.insert(*one_letter_word);
        }
    }

    for missing_one_letter_word in missing_one_letter_words.iter() {
        trace!("Missing expected word '{}'.", missing_one_letter_word);
    }

    for (key, value) in missing_one_letter_words_frequeny.iter() {
        trace!("'{}' word found but not expected with the frequeny {}.", key, value);
    }

    return (missing_one_letter_words, missing_one_letter_words_frequeny);
}

fn calculate_one_letter_words_frequeny(words: Vec<String>) -> HashMap<char, u32> {
    let mut one_letter_words_frequeny: HashMap<char, u32> = HashMap::new();

    for word in &words {
        if word.len() == 1 {
            let counter = one_letter_words_frequeny.entry(word.chars().next().unwrap()).or_insert(0);
            *counter += 1;
        }
    }

    for (key, value) in one_letter_words_frequeny.iter() {
        trace!("Word '{}' was found {} times.", key, value);
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
