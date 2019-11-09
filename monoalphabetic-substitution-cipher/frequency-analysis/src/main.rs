use regex::Regex;
use std::collections::HashMap;
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "frequency-analysis",
    about = "A rust implementation of a frequency analysis technique upon monoalphabetic substitution ciphers."
)]
struct Args {
    #[structopt(
        long = "input",
        help = "The path to a file containing the ciphertext to perform frequency analysis upon."
    )]
    input: String,
}

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static EXPECTED_LETTER_FREQUENCY: [char; 26] = [
    'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'u', 'c', 'm', 'w', 'f', 'g', 'y', 'p', 'b', 'v', 'k', 'j', 'x', 'q', 'z',
];

fn main() {
    let args = Args::from_args();
    let ciphertext: String = read_file(args.input);
    let plaintext: String = frequency_analysis(ciphertext);
    print!("{}", plaintext);
}

fn frequency_analysis(ciphertext: String) -> String {
    let mut letter_frequency: HashMap<char, u32> = calculate_letter_frequency(&ciphertext);
    let mut plaintext_chars: Vec<char> = ciphertext.clone().chars().collect();

    for i in 0..26 {
        let cipher_character: char = get_next_most_frequent(&letter_frequency);
        letter_frequency.remove(&cipher_character);
        plaintext_chars = replace_all_occurances(&ciphertext, plaintext_chars, cipher_character, EXPECTED_LETTER_FREQUENCY[i]);
    }

    return plaintext_chars.iter().collect();
}

fn read_file(filename: String) -> String {
    let file_contents = fs::read_to_string(filename).expect("Unable to read file.");

    return file_contents.to_ascii_lowercase();
}

fn calculate_letter_frequency(ciphertext: &str) -> HashMap<char, u32> {
    let mut letter_frequency: HashMap<char, u32> = HashMap::new();

    for (_index, character) in ciphertext.chars().enumerate() {
        if ALPHABET.contains(&character) {
            let counter = letter_frequency.entry(character).or_insert(0);
            *counter += 1;
        }
    }

    return letter_frequency;
}

fn get_next_most_frequent(letter_frequency: &HashMap<char, u32>) -> char {
    let mut cipher_character: char = *(letter_frequency.keys().next()).unwrap();
    let mut largest_value = *(letter_frequency.get(&cipher_character)).unwrap();

    for key in letter_frequency.keys() {
        if *(letter_frequency.get(key).unwrap()) > largest_value {
            cipher_character = *key;
            largest_value = *(letter_frequency.get(key)).unwrap();
        }
    }

    return cipher_character;
}

fn replace_all_occurances(orginal_copy: &str, mut modifying_copy: Vec<char>, replacing: char, replace_with: char) -> Vec<char> {
    for (index, character) in orginal_copy.chars().enumerate() {
        if character == replacing {
            modifying_copy[index] = replace_with;
        }
    }

    return modifying_copy;
}

fn get_all_words(sentence: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut sentence: String = replace_all_non_alphabet(sentence);
    sentence = remove_all_extra_spaces(sentence);

    for word in sentence.split(' ') {
        words.push(String::from(word));
    }

    return words;
}

fn replace_all_non_alphabet(replacing: &str) -> String {
    let mut returning = String::from(replacing);

    for (_index, character) in replacing.chars().enumerate() {
        if !ALPHABET.contains(&character) {
            returning = returning.replace(character, " ");
        }
    }

    return returning;
}

fn remove_all_extra_spaces(replacing: String) -> String {
    let returning = replacing.trim();
    let regex = Regex::new(r"\s+").unwrap();
    return regex.replace_all(returning, " ").into_owned();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(
        string,
        expected,
        case("aabac", [('a', 3),('b', 1),('c', 1)].iter().cloned().collect()),
        case("abbdab", [('a', 2),('b', 3),('d', 1)].iter().cloned().collect()),
        case("ccac", [('c', 3),('a', 1)].iter().cloned().collect()),
    )]
    fn test_calculate_letter_frequency(string: &str, expected: HashMap<char, u32>) {
        //when
        let letter_frequency = calculate_letter_frequency(string);

        //then
        assert_eq!(expected, letter_frequency);
    }

    #[rstest_parametrize(
        letter_frequency,
        expected,
        case([('a', 13),('b', 73),('c', 52)].iter().cloned().collect(), 'b'),
        case([('a', 13)].iter().cloned().collect(), 'a'),
        case([('a', 0),('b', 27),('c', 49)].iter().cloned().collect(), 'c'),
    )]
    fn test_get_next_most_frequent(letter_frequency: HashMap<char, u32>, expected: char) {
        //when
        let most_frequent_letter = get_next_most_frequent(&letter_frequency);

        //then
        assert_eq!(expected, most_frequent_letter);
    }

    #[rstest_parametrize(
        orginal,
        expected,
        replacing,
        replace_with,
        case("abcde", "ebcde", 'a', 'e'),
        case("abcde", "abcde", 'z', 'e'),
        case("ab cda", "eb cde", 'a', 'e')
    )]
    fn test_replace_all_occurances(orginal: &str, expected: &str, replacing: char, replace_with: char) {
        //given
        let expected: Vec<char> = expected.chars().collect();

        //when
        let returned = replace_all_occurances(&orginal, orginal.clone().chars().collect(), replacing, replace_with);

        //then
        assert_eq!(expected, returned);
    }

    #[rstest_parametrize(
        sentence,
        expected,
        case("this, is.", vec!["this".to_string(), "is".to_string()]),
        case("inside (brackets).", vec!["inside".to_string(), "brackets".to_string()]),
        case("full. stop, nope. ", vec!["full".to_string(), "stop".to_string(), "nope".to_string()])
    )]
    fn test_get_all_words(sentence: &str, expected: Vec<String>) {
        //when
        let returned = get_all_words(&sentence);

        //then
        assert_eq!(expected, returned);
    }

    #[rstest_parametrize(
        replacing,
        expected,
        case(" this!. is  ", " this   is  "),
        case("(word)", " word "),
        case(" end(game).", " end game  ")
    )]
    fn test_replace_all_non_alphabet(replacing: &str, expected: &str) {
        //when
        let returned = replace_all_non_alphabet(replacing);

        //then
        assert_eq!(expected.to_string(), returned);
    }

    #[rstest_parametrize(
        replacing,
        expected,
        case(" this  is  ", "this is"),
        case("   let try   ", "let try"),
        case("this  is   a ", "this is a")
    )]
    fn test_remove_all_extra_spaces(replacing: &str, expected: &str) {
        //when
        let returned = remove_all_extra_spaces(replacing.to_string());

        //then
        assert_eq!(expected.to_string(), returned);
    }
}
