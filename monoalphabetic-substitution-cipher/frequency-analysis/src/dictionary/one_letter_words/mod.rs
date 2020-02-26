use std::collections::HashMap;
use std::collections::HashSet;
use super::helper as dictionary_helper;
use super::super::helper;

lazy_static! {
    static ref ONE_LETTER_WORDS: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('i', 2);
        m.insert('a', 1);
        m
    };
}

pub fn one_letter_word_dictionary_corrections(plaintext: String) -> String {
    let one_letter_words_frequeny = dictionary_helper::calculate_one_letter_words_frequeny(dictionary_helper::get_all_words(&plaintext));
    let (missing_one_letter_words, unexpected_one_letter_words_frequeny) = get_missing_one_letter_words(one_letter_words_frequeny); 

    return missing_one_letter_words_corrections(plaintext, missing_one_letter_words, unexpected_one_letter_words_frequeny);
}

fn missing_one_letter_words_corrections(
    plaintext: String,
    mut missing_one_letter_words: HashSet<char>,
    mut unexpected_one_letter_words_frequeny: HashMap<char, u32>,
) -> String {
    let mut predicted_plaintext = plaintext.chars().collect();
    let unexpected_one_letter_words_frequeny_length = unexpected_one_letter_words_frequeny.len();

    for _i in 0..unexpected_one_letter_words_frequeny_length {
        let next_most_frequent_unexpect_word = helper::get_next_most_frequent(&unexpected_one_letter_words_frequeny);
        trace!("Next most frequent unexpected word is '{}'.", next_most_frequent_unexpect_word);
 
        let next_most_frequent_missing_word = get_next_most_frequent_missing_word(&missing_one_letter_words);
        trace!("Next most frequent expected word is '{}'.", next_most_frequent_missing_word);

        trace!("Switching the characters {} and {}.", next_most_frequent_unexpect_word, next_most_frequent_missing_word);
        predicted_plaintext = helper::replace_all_occurances(&plaintext, predicted_plaintext, next_most_frequent_unexpect_word, next_most_frequent_missing_word);
        predicted_plaintext = helper::replace_all_occurances(&plaintext, predicted_plaintext, next_most_frequent_missing_word, next_most_frequent_unexpect_word);

        missing_one_letter_words.remove(&next_most_frequent_missing_word);
        unexpected_one_letter_words_frequeny.remove(&next_most_frequent_unexpect_word);
    }

    return predicted_plaintext.into_iter().collect();
}

fn get_next_most_frequent_missing_word(
    missing_one_letter_words: &HashSet<char>
    ) -> char {
    let mut working_missing_one_letter_words_frequeny:HashMap<char, u32> = HashMap::new();

    for missing_one_letter_word in missing_one_letter_words {
        let frequeny = ONE_LETTER_WORDS.get(&missing_one_letter_word).unwrap();
        working_missing_one_letter_words_frequeny.insert(*missing_one_letter_word, *frequeny);
    }
    
    return helper::get_next_most_frequent(&working_missing_one_letter_words_frequeny); 
}

fn get_missing_one_letter_words(one_letter_words_frequeny: HashMap<char, u32>) -> (HashSet<char>, HashMap<char, u32>) {
    let mut unexpected_one_letter_words_frequeny = HashMap::new();
    let mut missing_one_letter_words: HashSet<char> = ONE_LETTER_WORDS.keys().cloned().collect();
    let mut working_missing_one_letter_words_frequeny = one_letter_words_frequeny.clone();
    let mut missing_one_letter_words_count = one_letter_words_frequeny.len();
    let one_letter_words_hashset: HashSet<char> = ONE_LETTER_WORDS.keys().cloned().collect();

    if missing_one_letter_words_count > 2 {
        missing_one_letter_words_count = 2;
    }

    for _i in 0..missing_one_letter_words_count {
        let next_most_frequent_one_letter_word = helper::get_next_most_frequent(&working_missing_one_letter_words_frequeny);

        if one_letter_words_hashset.contains(&next_most_frequent_one_letter_word) {
            missing_one_letter_words.remove(&next_most_frequent_one_letter_word);
        } else {
            unexpected_one_letter_words_frequeny.insert(
                next_most_frequent_one_letter_word,
                *working_missing_one_letter_words_frequeny
                    .get(&next_most_frequent_one_letter_word)
                    .unwrap(),
            );
        }

        working_missing_one_letter_words_frequeny.remove(&next_most_frequent_one_letter_word);
    }

    for missing_one_letter_word in missing_one_letter_words.iter() { 
        trace!("Missing expected word '{}'.", missing_one_letter_word);
    }

    for (key, value) in unexpected_one_letter_words_frequeny.iter() {
        trace!("'{}' word found but not expected with the frequeny {}.", key, value);
    }

    return (missing_one_letter_words, unexpected_one_letter_words_frequeny);
}

#[cfg(test)]
mod tests;
