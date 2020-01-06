use super::*;
use rstest::rstest_parametrize;

#[rstest_parametrize(plaintext, expected, case("a niw sintinci e maki", "a new sentence i make"))]
fn test_one_letter_word_dictionary_corrections(plaintext: &str, expected: &str) {
    //when
    let corrected = one_letter_word_dictionary_corrections(plaintext.to_string());

    //then
    assert_eq!(expected, corrected);
}

#[rstest_parametrize(
        words,
        expected,
        case(vec!["a".to_string(), "be".to_string(), "a".to_string(), "and".to_string(), "b".to_string()], [("a".to_string(), 2),("b".to_string(), 1)].iter().cloned().collect()),
        case(vec![], [].iter().cloned().collect()),
        case(vec!["lets".to_string(), "be".to_string(), "a".to_string(), "and".to_string()], [("a".to_string(), 1)].iter().cloned().collect()),
    )]
fn test_calculate_one_letter_words_frequeny(words: Vec<String>, expected: HashMap<String, u32>) {
    //when
    let one_letter_words_frequeny = calculate_one_letter_words_frequeny(words);

    //then
    assert_eq!(expected, one_letter_words_frequeny);
}

#[rstest_parametrize(
        sentence,
        expected,
        case("this, is. it's  ", vec!["this", "is", "it's"]),
        case("inside (brackets).", vec!["inside", "brackets"]),
        case("full. stop, nope. ", vec!["full", "stop", "nope"])
    )]
fn test_get_all_words(sentence: &str, expected: Vec<&str>) {
    //when
    let words = get_all_words(&sentence);

    //then
    assert_eq!(expected, words);
}

#[rstest_parametrize(
    replacing,
    expected,
    case(" this!. is  ", " this   is  "),
    case("(word)", " word "),
    case(" it's", " it's")
)]
fn test_replace_all_non_alphabet(replacing: &str, expected: &str) {
    //when
    let alphabet_only = replace_all_non_alphabet(replacing);

    //then
    assert_eq!(expected, alphabet_only);
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
    let spaces_removed = remove_all_extra_spaces(replacing.to_string());

    //then
    assert_eq!(expected, spaces_removed);
}
