use structopt::StructOpt;

mod helper;
mod frequency_analysis;
mod dictionary;

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


fn main() {
    let args = Args::from_args();
    let mut plaintext: String = frequency_analysis::frequency_analysis(helper::read_file(args.input));
    plaintext = dictionary::one_letter_word_dictionary_corrections(plaintext);
    print!("{}", plaintext);
}
