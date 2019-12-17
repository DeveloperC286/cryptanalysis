use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "frequency-analysis",
    about = "A rust implementation of a frequency analysis technique upon monoalphabetic substitution ciphers."
)]
struct Args {
    #[structopt(
        short = "i",
        long = "input",
        help = "The path to a file containing the ciphertext to perform frequency analysis upon."
    )]
    input: String,
}

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static EXPECTED_LETTER_FREQUENCY: [char; 26] = [
    'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u', 'c', 'm', 'w', 'f', 'g', 'y', 'p', 'b', 'v', 'k', 'j', 'x', 'q', 'z',
];

fn main() {
    let args = Args::from_args();
    let mut plaintext: String = frequency_analysis(read_file(args.input));
    plaintext = one_letter_word_dictionary_corrections(plaintext);
    print!("{}", plaintext);
}

fn frequency_analysis(ciphertext: String) -> String {
    let mut letter_frequency: HashMap<char, u32> = calculate_letter_frequency(&ciphertext);
    let mut plaintext_chars: Vec<char> = ciphertext.chars().collect();

    for i in 0..letter_frequency.len() {
        let cipher_character: char = get_next_most_frequent(&letter_frequency);
        letter_frequency.remove(&cipher_character);
        plaintext_chars = replace_all_occurances(&ciphertext, plaintext_chars, cipher_character, EXPECTED_LETTER_FREQUENCY[i]);
    }

    return plaintext_chars.iter().collect();
}

fn read_file(filename: String) -> String {
    let file_contents = fs::read_to_string(filename).expect("Unable to read file.");
    return file_contents.to_ascii_lowercase();
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

fn one_letter_word_dictionary_corrections(plaintext: String) -> String {
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
