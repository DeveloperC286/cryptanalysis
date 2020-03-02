use super::*;
use rstest::rstest;

#[rstest(
    plaintext, 
    expected, 
    case("a niw sintinci e maki", "a new sentence i make"), 
    case("i collect it", "i collect it"),
    case("i dftf ailes in f vflid stfte aollowing f crfsh", "i data files in a valid state following a crash")
)] 
fn test_one_letter_word_dictionary_corrections(plaintext: &str, expected: &str) {
    //when
    let corrected = one_letter_word_dictionary_corrections(plaintext.to_string());

    //then
    assert_eq!(expected, corrected);
}

#[rstest(
    plaintext,
    missing_one_letter_words,
    missing_one_letter_words_frequeny,
    expected_correction,
    case(
        "a niw sintinci e maki".to_string(),
        [('i')].iter().cloned().collect(),
        [('e', 1)].iter().cloned().collect(),
        "a new sentence i make".to_string()
    ),
    case(
        "i collect it".to_string(),
        [].iter().cloned().collect(),
        [].iter().cloned().collect(),
        "i collect it".to_string() 
    ),
    case(
        "i dftf ailes in f vflid stfte aollowing f crfsh".to_string(),
        [('a')].iter().cloned().collect(),
        [('f', 2)].iter().cloned().collect(),
        "i data files in a valid state following a crash".to_string()
    )
)]
fn test_missing_one_letter_words_corrections(
    plaintext: String,
    missing_one_letter_words: HashSet<char>,
    missing_one_letter_words_frequeny: HashMap<char, u32>,
    expected_correction: String,
) {
    //when
    let returned_correction = missing_one_letter_words_corrections(plaintext, missing_one_letter_words, missing_one_letter_words_frequeny);

    //then
    assert_eq!(expected_correction, returned_correction);
}

#[rstest(
    one_letter_words_frequeny,
    expected_missing_one_letter_words,
    expected_missing_one_letter_words_frequeny,
    case(
        [('a', 3), ('e', 1)].iter().cloned().collect(),
        [('i')].iter().cloned().collect(),
        [('e', 1)].iter().cloned().collect()
    ),
    case(
        [('t', 2), ('e', 3)].iter().cloned().collect(),
        [('a'), ('i')].iter().cloned().collect(),
        [('t', 2), ('e', 3)].iter().cloned().collect()
    ),
    case(
        [('a', 5), ('i', 3)].iter().cloned().collect(),
        [].iter().cloned().collect(),
        [].iter().cloned().collect()
    ),
    case(
        [('n', 5), ('a', 2), ('i', 92), ('o', 97), ('e', 1), ('y', 3), ('p', 2), ('t', 4)].iter().cloned().collect(),
        ['a'].iter().cloned().collect(),
        [('o', 97)].iter().cloned().collect()
    )
)]
fn test_get_missing_one_letter_words(
    one_letter_words_frequeny: HashMap<char, u32>,
    expected_missing_one_letter_words: HashSet<char>,
    expected_missing_one_letter_words_frequeny: HashMap<char, u32>,
) {
    //when
    let (returned_missing_one_letter_words, returned_missing_one_letter_words_frequeny) =
        get_missing_one_letter_words(one_letter_words_frequeny);

    //then
    assert_eq!(expected_missing_one_letter_words, returned_missing_one_letter_words);
    assert_eq!(
        expected_missing_one_letter_words_frequeny,
        returned_missing_one_letter_words_frequeny
    );
}
