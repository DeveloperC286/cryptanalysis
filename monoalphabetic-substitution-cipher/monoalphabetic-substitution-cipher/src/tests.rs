use super::*;
use rstest::rstest_parametrize;

#[rstest_parametrize(
    key,
    expected,
    case("otduxbylwjmqifarkzphencvgs", "ofwcunytmjqhkvaslpzbdxiegr"),
    case("pqgitynxwrovjlkmfsduhzebca", "zxyswqcudmonpgkabjretlihfv"),
    case("nkczrjsyfqtliaxhuowvebdmpg", "nvcwuizpmfblxaryjegkqtsohd")
)]
fn test_invert_key(expected: &str, key: &str) {
    //when
    let inverted_key = invert_key(key.chars().collect());

    //then
    let expected_vector: Vec<char> = expected.chars().collect();
    assert_eq!(expected_vector, inverted_key);
}

#[rstest_parametrize(
    key,
    enciphering,
    expected,
    case(
        "gqcerpnjmhtivalosbzwxdkfyu",
        "defend the east wall of the castle",
        "erprae wjr rgzw kgii lp wjr cgzwir"
    ),
    case(
        "ouknpajzbhlvdrqcsxftiemwgy",
        "the simple substitution cipher is a cipher that has been in use for many hundreds of years",
        "tzp fbdcvp fiuftbtitbqr kbczpx bf o kbczpx tzot zof uppr br ifp aqx dorg zirnxpnf qa gpoxf"
    ),
    case(
        "ypzvihmcgqobntajdwkeruxfls",
        "the simple substitution cipher is quite easy to break",
        "eci kgnjbi krpkegeregat zgjciw gk drgei iykl ea pwiyo"
    )
)]
fn test_encipher(key: &str, enciphering: &str, expected: &str) {
    //when
    let returned = encipher(key.chars().collect(), enciphering.to_string());

    //then
    assert_eq!(expected, returned);
}
