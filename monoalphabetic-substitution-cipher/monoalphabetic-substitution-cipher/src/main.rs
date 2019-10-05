#[macro_use]
extern crate lazy_static;

use std::process::exit;
use std::fs;
use structopt::StructOpt;
use std::collections::HashMap;

lazy_static! {
    static ref ALPHABET: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('a', 1);
        m.insert('b', 2);
        m.insert('c', 3);
        m.insert('d', 4);
        m.insert('e', 5);
        m.insert('f', 6);
        m.insert('g', 7);
        m.insert('h', 8);
        m.insert('i', 9);
        m.insert('j', 10);
        m.insert('k', 11);
        m.insert('l', 12);
        m.insert('m', 13);
        m.insert('n', 14);
        m.insert('o', 15);
        m.insert('p', 16);
        m.insert('q', 17);
        m.insert('r', 18);
        m.insert('s', 19);
        m.insert('t', 20);
        m.insert('u', 21);
        m.insert('v', 22);
        m.insert('w', 23);
        m.insert('x', 24);
        m.insert('y', 25);
        m.insert('z', 26);
        m
    };
}

#[derive(Debug, StructOpt)]
#[structopt(name = "monoalphabetic-substitution-cipher", about = "A rust implementation of a monoalphabetic substitution cipher.")]
struct Args {
    #[structopt(long = "key", help = "The path to a file containing the key to use in the substitution.")]
    key: String,

    #[structopt(long = "input", help = "The path to a file containing the text to be used as input to the substitution cipher.")] 
    input: String,

    #[structopt(long = "decipher", help = "A flag to specify if the file content should be deciphered instead of enciphered.")]
    decipher: bool,
}

static ASCII: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn main() { 
    let args = Args::from_args();

    let file_contents = read_file(args.input);
    let key = validate_key(args.key);

    let subsituted_file_contents;

    if args.decipher {
        subsituted_file_contents = encipher(invert_key(key), file_contents);
    } else {
        subsituted_file_contents = encipher(key, file_contents);
    } 

    println!("{}", subsituted_file_contents);
}

fn invert_key(key:Vec<char>) -> Vec<char> {
    let mut inverting_key = ASCII.clone().to_vec();

    for x in 0..key.len() {
        let i = ALPHABET.get(&key[x]).unwrap()-1;
        inverting_key[i as usize] = ASCII[x];
    }

    return inverting_key;
}

fn encipher(key:Vec<char>, file_contents:String) -> String { 
    let mut subsituting_file_contents: Vec<char> = file_contents.chars().collect();

    for x in 0..26 {  
        for (index, character) in file_contents.chars().enumerate() { 
            if character == ASCII[x] {
                subsituting_file_contents[index] = key[x];  
            }
        } 
    } 

    return subsituting_file_contents.iter().collect();
}

fn read_file(filename:String) -> String {
    let file_contents = fs::read_to_string(filename)
        .expect("Unable to read file.");

    return file_contents.to_ascii_lowercase();
}

fn validate_key(key:String) -> Vec<char> {
    let key:String = (read_file(key)).trim().to_string(); 

    if key.len() != 26 {
        println!("The key must be of length 26.");
         exit(1);
    }

    return key.chars().collect();
}

#[cfg(test)]
mod tests { 
    use super::*;
    use rstest::rstest_parametrize;


    #[rstest_parametrize(expected, key,
        case("otduxbylwjmqifarkzphencvgs", "ofwcunytmjqhkvaslpzbdxiegr"),
        case("pqgitynxwrovjlkmfsduhzebca", "zxyswqcudmonpgkabjretlihfv"),
        case("nkczrjsyfqtliaxhuowvebdmpg", "nvcwuizpmfblxaryjegkqtsohd"),
    )]
    fn test_invert_key(expected:&str, key:&str) {  
        //when
        let inverted_key = invert_key(key.chars().collect());
        let expected_vector:Vec<char>= expected.chars().collect();

        //then
        assert_eq!(expected_vector, inverted_key);
    }
}
