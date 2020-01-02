use super::super::helper;
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
    let plaintext = frequency_analysis(helper::read_file(ciphertext.to_string()));

    //then
    assert_eq!(helper::read_file(expected.to_string()), plaintext);
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
