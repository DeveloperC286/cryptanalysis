use super::helper::{ALPHABET, EXPECTED_LETTER_FREQUENCY};
use std::collections::HashMap;

pub fn frequency_analysis(ciphertext: String) -> String {
    let mut letter_frequency: HashMap<char, u32> = calculate_letter_frequency(&ciphertext);
    let mut plaintext_chars: Vec<char> = ciphertext.chars().collect();

    for i in 0..letter_frequency.len() {
        let cipher_character: char = get_next_most_frequent(&letter_frequency);
        letter_frequency.remove(&cipher_character);
        plaintext_chars = replace_all_occurances(&ciphertext, plaintext_chars, cipher_character, EXPECTED_LETTER_FREQUENCY[i]);
    }

    return plaintext_chars.iter().collect();
}

fn calculate_letter_frequency(ciphertext: &str) -> HashMap<char, u32> {
    let mut letter_frequency: HashMap<char, u32> = HashMap::new();

    for (_index, character) in ciphertext.chars().enumerate() {
        if ALPHABET.contains(&character) {
            let counter = letter_frequency.entry(character).or_insert(0);
            *counter += 1;
        }
    }

    return letter_frequency;
}

fn get_next_most_frequent(letter_frequency: &HashMap<char, u32>) -> char {
    let mut cipher_character: char = *(letter_frequency.keys().next()).unwrap();
    let mut largest_value = *(letter_frequency.get(&cipher_character)).unwrap();

    for key in letter_frequency.keys() {
        if *(letter_frequency.get(key).unwrap()) > largest_value {
            cipher_character = *key;
            largest_value = *(letter_frequency.get(key)).unwrap();
        }
    }

    return cipher_character;
}

fn replace_all_occurances(orginal_copy: &str, mut modifying_copy: Vec<char>, replacing: char, replace_with: char) -> Vec<char> {
    for (index, character) in orginal_copy.chars().enumerate() {
        if character == replacing {
            modifying_copy[index] = replace_with;
        }
    }

    return modifying_copy;
}

#[cfg(test)]
mod tests;
