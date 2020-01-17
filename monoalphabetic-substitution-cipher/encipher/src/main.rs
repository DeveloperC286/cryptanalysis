extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use monoalphabetic_substitution_cipher_encipher::{encipher, invert_key};
use std::fs;
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "monoalphabetic-substitution-cipher",
    about = "A rust implementation of a monoalphabetic substitution cipher."
)]
struct Args {
    #[structopt(
        short = "k",
        long = "key",
        help = "The path to a file containing the key to use in the substitution."
    )]
    key: String,

    #[structopt(
        short = "i",
        long = "input",
        help = "The path to a file containing the text to be used as input to the substitution cipher."
    )]
    input: String,

    #[structopt(
        short = "o",
        long = "output",
        help = "The path to a file containing the text output from the substitution cipher."
    )]
    output: Option<String>,

    #[structopt(
        long = "decipher",
        help = "A flag to specify if the file content should be deciphered instead of enciphered."
    )]
    decipher: bool,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::from_args();

    let file_contents = read_file(&args.input);
    let key = validate_key(&args.key);

    let subsituted_file_contents;

    if args.decipher {
        info!("Deciphering '{}' with the key '{}'.", args.input, args.key);
        subsituted_file_contents = encipher(invert_key(key), file_contents);
    } else {
        info!("Enciphering '{}' with the key '{}'.", args.input, args.key);
        subsituted_file_contents = encipher(key, file_contents);
    }

    if let Some(output) = args.output {
        info!("Writing output to '{}'.", output);
        write_file(&output, subsituted_file_contents);
    } else {
        print!("{}", subsituted_file_contents);
    }
}

fn read_file(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Unable to read file.").to_ascii_lowercase();
}

fn write_file(filename: &str, content: String) {
    fs::write(filename, content).expect("Unable to write file.");
}

fn validate_key(key: &str) -> Vec<char> {
    let key: String = (read_file(key)).trim().to_string();

    if key.len() != 26 {
        println!("The key must be of length 26.");
        exit(1);
    }

    return key.chars().collect();
}
