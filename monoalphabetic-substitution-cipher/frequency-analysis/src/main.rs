#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::fs;
use std::process::exit;

use clap::Parser;

mod dictionary;
mod frequency_analysis;
mod helper;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "The path to a file containing the ciphertext to perform frequency analysis upon."
    )]
    input: String,

    #[arg(
        short,
        long,
        help = "The path to a file containing the text output from the frequency analysis."
    )]
    output: Option<String>,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();
    info!("Performing frequency analysis upon '{}'.", args.input);
    let mut plaintext: String =
        frequency_analysis::frequency_analysis(helper::read_file(&args.input));
    plaintext = dictionary::one_letter_word_dictionary_corrections(plaintext);

    if let Some(output) = args.output {
        info!("Writing output to '{}'.", output);
        write_file(&output, plaintext);
    } else {
        print!("{}", plaintext);
    }
}

fn write_file(filename: &str, content: String) {
    match fs::write(filename, content) {
        Result::Ok(_success_message) => (),
        Result::Err(_error_message) => {
            error!("Unable to write the output to the file {}.", filename);
            exit(1);
        }
    }
}
