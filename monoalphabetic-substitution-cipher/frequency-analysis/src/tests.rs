use super::*;
use rstest::rstest_parametrize;

#[rstest_parametrize(
    ciphertext,
    expected,
    case(
        "../examples/The-Adventures-of-Sherlock-Holmes.ciphertext",
        "../examples/The-Adventures-of-Sherlock-Holmes.frequency-analysis"
    )
)]
fn test_frequency_analysis(ciphertext: &str, expected: &str) {
    //when
    let plaintext = frequency_analysis(read_file(ciphertext.to_string()));

    //then
    assert_eq!(read_file(expected.to_string()), plaintext);
}

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
    let replaced = replace_all_occurances(&orginal, orginal.chars().collect(), replacing, replace_with);

    //then
    assert_eq!(expected, replaced);
}

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
fn test_one_letter_words_frequeny(words: Vec<String>, expected: HashMap<String, u32>) {
    //when
    let one_letter_words_frequeny = one_letter_words_frequeny(words);

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
