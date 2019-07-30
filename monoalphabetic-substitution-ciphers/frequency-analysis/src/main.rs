use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    // Static variables.
    static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    static EXPECTED_LETTER_FREQUENCY: [char; 26] = ['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u', 'c', 'm', 'w', 'f', 'g', 'y', 'p', 'b', 'v', 'k', 'j', 'x', 'q', 'z'];

    // Read in ciphertext. 
    let args: Vec<String> = env::args().collect();

    let ciphertext_filename = &args[1];

    let ciphertext = fs::read_to_string(ciphertext_filename)
        .expect("Unable to read file.");

    // Setup frequency hashmap.
    let mut letter_count:HashMap<char, u32> = HashMap::new();

    // Count the frequency.
    for (_index, character) in ciphertext.to_ascii_lowercase().chars().enumerate() {
        if ASCII_LOWER.contains(&character) {
            let counter = letter_count.entry(character).or_insert(0);
            *counter += 1;
        }
    }

    let mut plaintext_chars:Vec<char> = ciphertext.to_ascii_lowercase().clone().chars().collect();

	// print out in order.
    for i in 0..26 {  
        let mut cipher_character:char = *(letter_count.keys().next()).unwrap();
        let mut largest_value = *(letter_count.get(&cipher_character)).unwrap();

        for key in letter_count.keys() {
            if *(letter_count.get(key).unwrap()) > largest_value {
                cipher_character = *key;
                largest_value = *(letter_count.get(key)).unwrap();
            }
        }
	
        // Replace chracter with expected.
        for (index, character) in ciphertext.to_ascii_lowercase().chars().enumerate() {
            if character == cipher_character {
                plaintext_chars[index] = EXPECTED_LETTER_FREQUENCY[i];
            }
        }

		letter_count.remove(&cipher_character);
    }

    let plaintext:String = plaintext_chars.iter().collect();

    println!("{}", plaintext);
}
