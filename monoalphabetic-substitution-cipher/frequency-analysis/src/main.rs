use structopt::StructOpt;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, StructOpt)]
#[structopt(name = "frequency-analysis", about = "A rust implementation of a frequency analysis technique upon monoalphabetic substitution ciphers.")]
struct Args {
    #[structopt(long = "input", help = "The path to a file containing the ciphertext to perform frequency analysis upon.")]
    input: String,
}

static ASCII: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

static EXPECTED_LETTER_FREQUENCY: [char; 26] = ['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u', 'c', 'm', 'w', 'f', 'g', 'y', 'p', 'b', 'v', 'k', 'j', 'x', 'q', 'z'];

fn main() {
    let args = Args::from_args(); 

    let ciphertext:String= read_file(args.input);
 
    let mut letter_frequency:HashMap<char, u32> = calculate_letter_frequency(&ciphertext);
    let mut plaintext_chars:Vec<char> = ciphertext.clone().chars().collect();

    for i in 0..26 {  
        let cipher_character:char = get_next_most_frequent(&letter_frequency);
    	letter_frequency.remove(&cipher_character);
        plaintext_chars = replace_all_occurances(&ciphertext, plaintext_chars, cipher_character, EXPECTED_LETTER_FREQUENCY[i]); 
    }

    let plaintext:String = plaintext_chars.iter().collect();
    println!("{}", plaintext);
}

fn read_file(filename:String) -> String {
    let file_contents = fs::read_to_string(filename)
        .expect("Unable to read file.");

    return file_contents.to_ascii_lowercase();
}

fn calculate_letter_frequency(ciphertext:&str) -> HashMap<char, u32> {
    let mut letter_frequency:HashMap<char, u32> = HashMap::new();

    for (_index, character) in ciphertext.chars().enumerate() {
        if ASCII.contains(&character) {
            let counter = letter_frequency.entry(character).or_insert(0);
            *counter += 1;
        }
    }

    return letter_frequency;
}

fn get_next_most_frequent(letter_frequency:&HashMap<char, u32>) -> char {
    let mut cipher_character:char = *(letter_frequency.keys().next()).unwrap();
    let mut largest_value = *(letter_frequency.get(&cipher_character)).unwrap();

    for key in letter_frequency.keys() {
        if *(letter_frequency.get(key).unwrap()) > largest_value {
            cipher_character = *key;
            largest_value = *(letter_frequency.get(key)).unwrap();
        }
    }

    return cipher_character;
}

fn replace_all_occurances(orginal_copy:&str, mut modifying_copy:Vec<char>, replacing:char, replace_with:char) -> Vec<char> { 
    for (index, character) in orginal_copy.chars().enumerate() {
        if character == replacing {
            modifying_copy[index] = replace_with; 
        }
    }

    return modifying_copy;
}
