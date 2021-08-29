use super::*;
use rstest::rstest;

#[rstest(
        letter_frequency,
        expected_most_frequent_letter,
        case([('a', 13),('b', 73),('c', 52)].iter().cloned().collect(), 'b'),
        case([('a', 13)].iter().cloned().collect(), 'a'),
        case([('a', 0),('b', 27),('c', 49)].iter().cloned().collect(), 'c'),
    )]
fn test_get_next_most_frequent(letter_frequency: HashMap<char, u32>, expected_most_frequent_letter: char) {
    //when
    let returned_most_frequent_letter = get_next_most_frequent(&letter_frequency);

    //then
    assert_eq!(expected_most_frequent_letter, returned_most_frequent_letter);
}

#[rstest(
    orginal,
    expected_letter_frequency,
    replacing,
    replace_with,
    case("abcde", "ebcde", 'a', 'e'),
    case("abcde", "abcde", 'z', 'e'),
    case("ab cda", "eb cde", 'a', 'e')
)]
fn test_replace_all_occurances(orginal: &str, expected_letter_frequency: &str, replacing: char, replace_with: char) {
    //given
    let expected_letter_frequency: Vec<char> = expected_letter_frequency.chars().collect();

    //when
    let returned_letter_frequency = replace_all_occurances(orginal, orginal.chars().collect(), replacing, replace_with);

    //then
    assert_eq!(expected_letter_frequency, returned_letter_frequency);
}
