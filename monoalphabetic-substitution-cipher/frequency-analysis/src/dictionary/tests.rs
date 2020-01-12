use super::*;
use rstest::rstest;

#[rstest(plaintext, expected, case("a niw sintinci e maki", "a new sentence i make"))]
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

#[rstest(
        words,
        expected_one_letter_words_frequeny,
        case(vec!["a".to_string(), "be".to_string(), "a".to_string(), "and".to_string(), "b".to_string()], [('a', 2),('b', 1)].iter().cloned().collect()),
        case(vec![], [].iter().cloned().collect()),
        case(vec!["lets".to_string(), "be".to_string(), "a".to_string(), "and".to_string()], [('a', 1)].iter().cloned().collect()),
    )]
fn test_calculate_one_letter_words_frequeny(words: Vec<String>, expected_one_letter_words_frequeny: HashMap<char, u32>) {
    //when
    let returned_one_letter_words_frequeny = calculate_one_letter_words_frequeny(words);

    //then
    assert_eq!(expected_one_letter_words_frequeny, returned_one_letter_words_frequeny);
}

#[rstest(
        sentence,
        expected_all_words,
        case("this, is. it's  ", vec!["this", "is", "it's"]),
        case("inside (brackets).", vec!["inside", "brackets"]),
        case("full. stop, nope. ", vec!["full", "stop", "nope"])
    )]
fn test_get_all_words(sentence: &str, expected_all_words: Vec<&str>) {
    //when
    let returned_all_words = get_all_words(&sentence);

    //then
    assert_eq!(expected_all_words, returned_all_words);
}

#[rstest(
    replacing,
    expected_alphabet_only,
    case(" this!. is  ", " this   is  "),
    case("(word)", " word "),
    case(" it's", " it's")
)]
fn test_replace_all_non_alphabet(replacing: &str, expected_alphabet_only: &str) {
    //when
    let returned_alphabet_only = replace_all_non_alphabet(replacing);

    //then
    assert_eq!(expected_alphabet_only, returned_alphabet_only);
}

#[rstest(
    replacing,
    expected_removed_spaces,
    case(" this  is  ", "this is"),
    case("   let try   ", "let try"),
    case("this  is   a ", "this is a")
)]
fn test_remove_all_extra_spaces(replacing: &str, expected_removed_spaces: &str) {
    //when
    let returned_removed_spaces = remove_all_extra_spaces(replacing.to_string());

    //then
    assert_eq!(expected_removed_spaces, returned_removed_spaces);
}
