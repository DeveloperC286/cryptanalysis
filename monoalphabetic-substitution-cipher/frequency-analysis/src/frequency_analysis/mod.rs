use std::collections::HashMap;

use super::helper;

pub fn frequency_analysis(ciphertext: String) -> String {
    let mut letter_frequency: HashMap<char, u32> = calculate_letter_frequency(&ciphertext);
    let mut plaintext_chars: Vec<char> = ciphertext.chars().collect();

    for i in 0..letter_frequency.len() {
        let cipher_character: char = helper::get_next_most_frequent(&letter_frequency);
        letter_frequency.remove(&cipher_character);
        plaintext_chars = helper::replace_all_occurances(
            &ciphertext,
            plaintext_chars,
            cipher_character,
            helper::EXPECTED_LETTER_FREQUENCY[i],
        );
    }

    plaintext_chars.iter().collect()
}

fn calculate_letter_frequency(ciphertext: &str) -> HashMap<char, u32> {
    let mut letter_frequency: HashMap<char, u32> = HashMap::new();

    for character in ciphertext.chars() {
        if helper::ALPHABET.contains(&character) {
            let counter = letter_frequency.entry(character).or_insert(0);
            *counter += 1;
        }
    }

    for (key, value) in letter_frequency.iter() {
        trace!("Letter '{}' was counted {} times.", key, value);
    }

    letter_frequency
}

#[cfg(test)]
mod tests;
