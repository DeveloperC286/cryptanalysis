use super::super::helper;
use super::*;
use rstest::rstest_parametrize;

#[rstest_parametrize(
    ciphertext_file,
    expected_frequency_analysis_text_file,
    case(
        "../examples/The-Adventures-of-Sherlock-Holmes.ciphertext",
        "../examples/The-Adventures-of-Sherlock-Holmes.frequency-analysis"
    )
)]
fn test_frequency_analysis(ciphertext_file: &str, expected_frequency_analysis_text_file: &str) {
    //when
    let returned_frequency_analysis_text = frequency_analysis(helper::read_file(ciphertext_file.to_string()));

    //then
    assert_eq!(
        helper::read_file(expected_frequency_analysis_text_file.to_string()),
        returned_frequency_analysis_text
    );
}

#[rstest_parametrize(
        string,
        expected_letter_frequency,
        case("aabac", [('a', 3),('b', 1),('c', 1)].iter().cloned().collect()),
        case("abbdab", [('a', 2),('b', 3),('d', 1)].iter().cloned().collect()),
        case("ccac", [('c', 3),('a', 1)].iter().cloned().collect()),
    )]
fn test_calculate_letter_frequency(string: &str, expected_letter_frequency: HashMap<char, u32>) {
    //when
    let returned_letter_frequency = calculate_letter_frequency(string);

    //then
    assert_eq!(expected_letter_frequency, returned_letter_frequency);
}
