use std::collections::HashMap;
use std::fs;

pub static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub static EXPECTED_LETTER_FREQUENCY: [char; 26] = [
    'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u', 'c', 'm', 'w', 'f', 'g', 'y', 'p', 'b', 'v', 'k', 'j', 'x', 'q', 'z',
];

pub fn read_file(filename: String) -> String {
    let file_contents = fs::read_to_string(filename).expect("Unable to read file.");
    return file_contents.to_ascii_lowercase();
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

    return cipher_character;
}

pub fn replace_all_occurances(orginal_copy: &str, mut modifying_copy: Vec<char>, replacing: char, replace_with: char) -> Vec<char> {
    for (index, character) in orginal_copy.chars().enumerate() {
        if character == replacing {
            modifying_copy[index] = replace_with;
        }
    }

    return modifying_copy;
}

#[cfg(test)]
mod tests;