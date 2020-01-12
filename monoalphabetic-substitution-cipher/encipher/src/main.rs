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
    let args = Args::from_args();

    let file_contents = read_file(args.input);
    let key = validate_key(args.key);

    let subsituted_file_contents;

    if args.decipher {
        subsituted_file_contents = encipher(invert_key(key), file_contents);
    } else {
        subsituted_file_contents = encipher(key, file_contents);
    }

    if let Some(output) = args.output {
        write_file(output, subsituted_file_contents);
    } else {
        print!("{}", subsituted_file_contents);
    }
}

fn read_file(filename: String) -> String {
    let file_contents = fs::read_to_string(filename).expect("Unable to read file.");

    return file_contents.to_ascii_lowercase();
}

fn write_file(filename: String, content: String) {
    fs::write(filename, content).expect("Unable to write file.");
}

fn validate_key(key: String) -> Vec<char> {
    let key: String = (read_file(key)).trim().to_string();

    if key.len() != 26 {
        println!("The key must be of length 26.");
        exit(1);
    }

    return key.chars().collect();
}
