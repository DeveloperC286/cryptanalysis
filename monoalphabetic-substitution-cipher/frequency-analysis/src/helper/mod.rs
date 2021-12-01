use std::collections::HashMap;
use std::fs;
use std::process::exit;

pub static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub static EXPECTED_LETTER_FREQUENCY: [char; 26] = [
    'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u', 'c', 'm', 'w', 'f', 'g', 'y', 'p',
    'b', 'v', 'k', 'j', 'x', 'q', 'z',
];

pub fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Result::Ok(file_content) => file_content.to_ascii_lowercase(),
        Result::Err(_error_message) => {
            error!("Unable to read from the file {}.", filename);
            exit(1);
        }
    }
}

pub fn get_next_most_frequent(letter_frequency: &HashMap<char, u32>) -> char {
    let mut cipher_character: char = *(letter_frequency.keys().next()).unwrap();
    let mut largest_value = *(letter_frequency.get(&cipher_character)).unwrap();

    for key in letter_frequency.keys() {
        if *(letter_frequency.get(key).unwrap()) > largest_value {
            cipher_character = *key;
            largest_value = *(letter_frequency.get(key)).unwrap();
        }
    }

    trace!("Next most frequent is '{}'.", cipher_character);
    cipher_character
}

pub fn replace_all_occurances(
    orginal_copy: &str,
    mut modifying_copy: Vec<char>,
    replacing: char,
    replace_with: char,
) -> Vec<char> {
    trace!("Replacing '{}' with '{}'.", replacing, replace_with);
    for (index, character) in orginal_copy.chars().enumerate() {
        if character == replacing {
            modifying_copy[index] = replace_with;
        }
    }

    modifying_copy
}

#[cfg(test)]
mod tests;
