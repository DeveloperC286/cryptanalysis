#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::fs;
use std::process::exit;

use clap::Parser;
use monoalphabetic_substitution_cipher_encipher::{encipher, invert_key};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "The path to a file containing the key to use in the substitution."
    )]
    key: String,

    #[arg(
        short,
        long,
        help = "The path to a file containing the text to be used as input to the substitution cipher."
    )]
    input: String,

    #[arg(
        short,
        long,
        help = "The path to write the output from the substitution cipher too."
    )]
    output: Option<String>,

    #[arg(
        long,
        help = "A flag to specify if the input file's content should be deciphered instead of enciphered."
    )]
    decipher: bool,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    let file_contents = read_file(&args.input);
    let key = validate_key(&args.key);

    let subsituted_file_contents = if args.decipher {
        info!("Deciphering '{}' with the key '{}'.", args.input, args.key);
        encipher(invert_key(key), file_contents)
    } else {
        info!("Enciphering '{}' with the key '{}'.", args.input, args.key);
        encipher(key, file_contents)
    };

    if let Some(output) = args.output {
        info!("Writing output to '{output}'.");
        write_file(&output, subsituted_file_contents);
    } else {
        print!("{subsituted_file_contents}");
    }
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Result::Ok(file_content) => file_content.to_ascii_lowercase(),
        Result::Err(_error_message) => {
            error!("Unable to read from the file {filename}.");
            exit(1);
        }
    }
}

fn write_file(filename: &str, content: String) {
    match fs::write(filename, content) {
        Result::Ok(_success_message) => (),
        Result::Err(_error_message) => {
            error!("Unable to write the output to the file {filename}.");
            exit(1);
        }
    }
}

fn validate_key(key: &str) -> Vec<char> {
    let key: String = (read_file(key)).trim().to_string();

    if key.len() != 26 {
        error!("The key must be of length 26.");
        exit(1);
    }

    key.chars().collect()
}
