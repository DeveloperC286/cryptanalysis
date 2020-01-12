use structopt::StructOpt;
use std::fs;

mod dictionary;
mod frequency_analysis;
mod helper;

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

    #[structopt(
        short = "o",
        long = "output",
        help = "The path to a file containing the text output from the frequency analysis."
    )]
    output: Option<String>,
}

fn main() {
    let args = Args::from_args();
    let mut plaintext: String = frequency_analysis::frequency_analysis(helper::read_file(args.input));
    plaintext = dictionary::one_letter_word_dictionary_corrections(plaintext);

    if let Some(output) = args.output {
        write_file(output, plaintext);
    } else {
        print!("{}", plaintext);
    }
}

fn write_file(filename: String, content: String) {
    fs::write(filename, content).expect("Unable to write file.");
}
