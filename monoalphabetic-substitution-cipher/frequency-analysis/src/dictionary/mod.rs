mod dictionary_helper;
mod one_letter_words;

pub fn one_letter_word_dictionary_corrections(plaintext: String) -> String {
    one_letter_words::one_letter_word_dictionary_corrections(plaintext)
}
