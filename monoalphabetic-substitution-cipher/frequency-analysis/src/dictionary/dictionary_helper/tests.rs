use super::*;
use rstest::rstest;

#[rstest(
        sentence,
        length,
        expected_one_letter_words_frequeny,
        case("a be a and b", 1, [('a', 2),('b', 1)].iter().cloned().collect()),
        case("", 1, [].iter().cloned().collect()),
        case("lets be a and", 1, [('a', 1)].iter().cloned().collect()),
)]
fn test_calculate_word_frequeny_with_length(sentence: &str, length: usize, expected_one_letter_words_frequeny: HashMap<char, u32>) {
    //when
    let returned_one_letter_words_frequeny = calculate_word_frequeny_with_length(sentence, length);

    //then
    assert_eq!(expected_one_letter_words_frequeny, returned_one_letter_words_frequeny);
}

#[rstest(
        sentence,
        expected_all_words,
        case("this, is. it's  ", vec!["this", "is", "its"]),
        case("inside (brackets).", vec!["inside", "brackets"]),
        case("full. stop, nope. ", vec!["full", "stop", "nope"]),
        case("example (e.g.)  ", vec!["example", "eg"])
)]
fn test_get_all_words(sentence: &str, expected_all_words: Vec<&str>) {
    //when
    let returned_all_words = get_all_words(sentence);

    //then
    assert_eq!(expected_all_words, returned_all_words);
}

#[rstest(
    replacing,
    expected_alphabet_only,
    case(" this!. is  ", " this is  "),
    case("(word)", "word"),
    case(" it's", " its"),
    case(" e.g. ", " eg ")
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
    case("this  is   a ", "this is a"),
    case("  e.g.  i.e.   ", "e.g. i.e.")
)]
fn test_remove_all_extra_spaces(replacing: &str, expected_removed_spaces: &str) {
    //when
    let returned_removed_spaces = remove_all_extra_spaces(replacing.to_string());

    //then
    assert_eq!(expected_removed_spaces, returned_removed_spaces);
}
