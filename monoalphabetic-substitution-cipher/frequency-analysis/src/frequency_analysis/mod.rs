use super::helper::{get_next_most_frequent, replace_all_occurances, ALPHABET, EXPECTED_LETTER_FREQUENCY};
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

    for (key, value) in letter_frequency.iter() {
        trace!("Letter '{}' was found {} times.", key, value);
    }
 
    return letter_frequency;
}

#[cfg(test)]
mod tests;
