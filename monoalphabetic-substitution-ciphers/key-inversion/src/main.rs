use std::env;
use std::process::exit;

fn main() {
    // Static variables.
    static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    // Start
    let args: Vec<String> = env::args().collect();

    let key = &args[1];

    if key.len() != 26 {
        println!("The key must be of length 26.");
        exit(1);
    }
 
    let key_vector:Vec<char> = key.chars().collect();  

    let mut inverting_key = ASCII_LOWER.clone();

    for x in 0..26 {
        for i in 0..26 { 
            if key_vector[x] == ASCII_LOWER[i] {
                inverting_key[i] = ASCII_LOWER[x]
            }
        }
    }

    let inverted_key:String = inverting_key.iter().collect();
    println!("{}", inverted_key);
}
