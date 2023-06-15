#[rustfmt::skip]
use seven_seg::*;

#[test]
fn sevseg_one_digit_zero() {
    assert_eq!(
        sevseg_one("0"),
        Some("┏━━━┓\n\
              ┃   ┃\n\
              ┃   ┃\n\
              ┃   ┃\n\
              ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_one() {
    assert_eq!(
        sevseg_one("1"),
        Some("    ╻\n    ┃\n    ┃\n    ┃\n    ╹\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_two() {
    assert_eq!(
        sevseg_one("2"),
        Some("╺━━━┓\n    ┃\n┏━━━┛\n┃    \n┗━━━╸\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_three() {
    assert_eq!(
        sevseg_one("3"),
        Some("╺━━━┓\n    ┃\n╺━━━┫\n    ┃\n╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_four() {
    assert_eq!(
        sevseg_one("4"),
        Some("╻   ╻\n┃   ┃\n┗━━━┫\n    ┃\n    ╹\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_five() {
    assert_eq!(
        sevseg_one("5"),
        Some("┏━━━╸\n┃    \n┗━━━┓\n    ┃\n╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_six() {
    assert_eq!(
        sevseg_one("6"),
        Some("┏━━━╸\n\
              ┃    \n\
              ┣━━━┓\n\
              ┃   ┃\n\
              ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_seven() {
    assert_eq!(
        sevseg_one("7"),
        Some("╺━━━┓\n    ┃\n    ┃\n    ┃\n    ╹\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_eight() {
    assert_eq!(
        sevseg_one("8"),
        Some("┏━━━┓\n\
              ┃   ┃\n\
              ┣━━━┫\n\
              ┃   ┃\n\
              ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_nine() {
    assert_eq!(
        sevseg_one("9"),
        Some("┏━━━┓\n┃   ┃\n┗━━━┫\n    ┃\n╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_one_digit_ten() {
    assert_eq!(
        sevseg_one("-"),
        Some("     \n     \n╺━━━╸\n     \n     \n".to_string())
    );
}

#[test]
fn sevseg_one_digit_a_none() {
    assert_eq!(sevseg_one("a"), None);
}

#[test]
fn sevseg_one_digit_len_none() {
    assert_eq!(sevseg_one("0124"), None);
}

#[test]
fn sevseg_one_digit_two_none() {
    assert_eq!(sevseg_one("-0"), None);
}

#[test]
fn sevseg_one_digit_empty() {
    assert_eq!(sevseg_one(""), None);
}

#[test]
fn sevseg_two_digit_zero() {
    assert_eq!(
        sevseg_two("00"),
        Some("┏━━━┓ ┏━━━┓\n\
              ┃   ┃ ┃   ┃\n\
              ┃   ┃ ┃   ┃\n\
              ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_one() {
    assert_eq!(
        sevseg_two("11"),
        Some("    ╻     ╻\n    ┃     ┃\n    ┃     ┃\n    ┃     ┃\n    ╹     ╹\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_two() {
    assert_eq!(
        sevseg_two("22"),
        Some("╺━━━┓ ╺━━━┓\n    ┃     ┃\n┏━━━┛ ┏━━━┛\n┃     ┃    \n┗━━━╸ ┗━━━╸\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_three() {
    assert_eq!(
        sevseg_two("33"),
        Some("╺━━━┓ ╺━━━┓\n    ┃     ┃\n╺━━━┫ ╺━━━┫\n    ┃     ┃\n╺━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_four() {
    assert_eq!(
        sevseg_two("44"),
        Some("╻   ╻ ╻   ╻\n┃   ┃ ┃   ┃\n┗━━━┫ ┗━━━┫\n    ┃     ┃\n    ╹     ╹\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_five() {
    assert_eq!(
        sevseg_two("55"),
        Some("┏━━━╸ ┏━━━╸\n┃     ┃    \n┗━━━┓ ┗━━━┓\n    ┃     ┃\n╺━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_six() {
    assert_eq!(
        sevseg_two("66"),
        Some("┏━━━╸ ┏━━━╸\n\
              ┃     ┃    \n\
              ┣━━━┓ ┣━━━┓\n\
              ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_seven() {
    assert_eq!(
        sevseg_two("77"),
        Some("╺━━━┓ ╺━━━┓\n    ┃     ┃\n    ┃     ┃\n    ┃     ┃\n    ╹     ╹\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_eight() {
    assert_eq!(
        sevseg_two("88"),
        Some("┏━━━┓ ┏━━━┓\n\
              ┃   ┃ ┃   ┃\n\
              ┣━━━┫ ┣━━━┫\n\
              ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_nine() {
    assert_eq!(
        sevseg_two("99"),
        Some("┏━━━┓ ┏━━━┓\n┃   ┃ ┃   ┃\n┗━━━┫ ┗━━━┫\n    ┃     ┃\n╺━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_eight_nine() {
    assert_eq!(
        sevseg_two("89"),
        Some("┏━━━┓ ┏━━━┓\n\
              ┃   ┃ ┃   ┃\n\
              ┣━━━┫ ┗━━━┫\n\
              ┃   ┃     ┃\n\
              ┗━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_empty_nine() {
    assert_eq!(
        sevseg_two("8"), 
        Some("┏━━━┓ ┏━━━┓\n\
              ┃   ┃ ┃   ┃\n\
              ┃   ┃ ┣━━━┫\n\
              ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_two_digit_ten() {
    assert_eq!(
        sevseg_two("--"),
        Some("           \n           \n╺━━━╸ ╺━━━╸\n           \n           \n".to_string())
    );
}

#[test]
fn sevseg_two_digit_empty_ten() {
    assert_eq!(sevseg_two("-"),
        Some("┏━━━┓      \n\
              ┃   ┃      \n\
              ┃   ┃ ╺━━━╸\n\
              ┃   ┃      \n\
              ┗━━━┛      \n".to_string())
        );
}

#[test]
fn sevseg_two_digit_empty() {
    assert_eq!(sevseg_two(""), None);
}

#[test]
fn sevseg_two_digit_a_none() {
    assert_eq!(sevseg_two("a"), None);
}

#[test]
fn sevseg_two_digit_len_none() {
    assert_eq!(sevseg_two("0124"), None);
}

#[test]
fn sevseg_two_digit_two_none() {
    assert_eq!(sevseg_two("--0"), None);
}

#[test]
fn sevseg_three_digit_zero() {
    assert_eq!(
        sevseg_three("000"),
        Some("┏━━━┓ ┏━━━┓ ┏━━━┓\n\
              ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_three_digit_one() {
    assert_eq!(
        sevseg_three("111"),
        Some("    ╻     ╻     ╻\n    ┃     ┃     ┃\n    ┃     ┃     ┃\n    ┃     ┃     ┃\n    ╹     ╹     ╹\n\
            ".to_string())
    );
}

#[test]
fn sevseg_three_digit_two() {
    assert_eq!(
        sevseg_three("222"),
        Some("╺━━━┓ ╺━━━┓ ╺━━━┓\n    ┃     ┃     ┃\n┏━━━┛ ┏━━━┛ ┏━━━┛\n┃     ┃     ┃    \n\
            ┗━━━╸ ┗━━━╸ ┗━━━╸\n".to_string())
    );
}

#[test]
fn sevseg_three_digit_three() {
    assert_eq!(
        sevseg_three("333"),
        Some("╺━━━┓ ╺━━━┓ ╺━━━┓\n    ┃     ┃     ┃\n╺━━━┫ ╺━━━┫ ╺━━━┫\n    ┃     ┃     ┃\n\
            ╺━━━┛ ╺━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_three_digit_four() {
    assert_eq!(
        sevseg_three("444"),
        Some("╻   ╻ ╻   ╻ ╻   ╻\n┃   ┃ ┃   ┃ ┃   ┃\n\
            ┗━━━┫ ┗━━━┫ ┗━━━┫\n    ┃     ┃     ┃\n    ╹     ╹     ╹\n".to_string())
    );
}

#[test]
fn sevseg_three_digit_five() {
    assert_eq!(
        sevseg_three("555"),
        Some("┏━━━╸ ┏━━━╸ ┏━━━╸\n┃     ┃     ┃    \n┗━━━┓ ┗━━━┓ ┗━━━┓\n    ┃     ┃     ┃\n\
            ╺━━━┛ ╺━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_three_digit_six() {
    assert_eq!(
        sevseg_three("666"),
        Some("┏━━━╸ ┏━━━╸ ┏━━━╸\n\
              ┃     ┃     ┃    \n\
              ┣━━━┓ ┣━━━┓ ┣━━━┓\n\
              ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_three_digit_seven() {
    assert_eq!(
        sevseg_three("777"),
        Some("╺━━━┓ ╺━━━┓ ╺━━━┓\n    ┃     ┃     ┃\n    ┃     ┃     ┃\n    ┃     ┃     ┃\n    ╹     ╹     ╹\n\
        ".to_string())
    );
}

#[test]
fn sevseg_three_digit_eight() {
    assert_eq!(
        sevseg_three("888"),
        Some("┏━━━┓ ┏━━━┓ ┏━━━┓\n\
              ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┣━━━┫ ┣━━━┫ ┣━━━┫\n\
              ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_three_digit_nine() {
    assert_eq!(
        sevseg_three("999"),
        Some("┏━━━┓ ┏━━━┓ ┏━━━┓\n┃   ┃ ┃   ┃ ┃   ┃\n┗━━━┫ ┗━━━┫ ┗━━━┫\n    ┃     ┃     ┃\n\
            ╺━━━┛ ╺━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_three_digit_ten() {
    assert_eq!(
        sevseg_three("---"),
        Some("                 \n                 \n\
            ╺━━━╸ ╺━━━╸ ╺━━━╸\n                 \n                 \n".to_string())
    );
}

#[test]
fn sevseg_three_digit_empty_ten() {
    assert_eq!(sevseg_three("-"),
        Some("┏━━━┓ ┏━━━┓      \n\
              ┃   ┃ ┃   ┃      \n\
              ┃   ┃ ┃   ┃ ╺━━━╸\n\
              ┃   ┃ ┃   ┃      \n\
              ┗━━━┛ ┗━━━┛      \n".to_string())
        );
}

#[test]
fn sevseg_three_digit_empty_five() {
    assert_eq!(sevseg_three("5"),
        Some("┏━━━┓ ┏━━━┓ ┏━━━╸\n\
              ┃   ┃ ┃   ┃ ┃    \n\
              ┃   ┃ ┃   ┃ ┗━━━┓\n\
              ┃   ┃ ┃   ┃     ┃\n\
              ┗━━━┛ ┗━━━┛ ╺━━━┛\n".to_string())
        );
}

#[test]
fn sevseg_three_digit_empty_one_two() {
    assert_eq!(sevseg_three("21"),
        Some("┏━━━┓ ╺━━━┓     ╻\n\
              ┃   ┃     ┃     ┃\n\
              ┃   ┃ ┏━━━┛     ┃\n\
              ┃   ┃ ┃         ┃\n\
              ┗━━━┛ ┗━━━╸     ╹\n".to_string())
    );
}

#[test]
fn sevseg_three_digit_one_a_two() {
    assert_eq!(sevseg_three("1a2"), None);
}

#[test]
fn sevseg_three_digit_one_two_a() {
    assert_eq!(sevseg_three("12a"), None);
}

#[test]
fn sevseg_three_digit_a_none() {
    assert_eq!(sevseg_three("a12"), None);
}

#[test]
fn sevseg_three_digit_len_none() {
    assert_eq!(sevseg_three("0124"), None);
}

#[test]
fn sevseg_three_digit_two_none() {
    assert_eq!(sevseg_three("----"), None);
}

#[test]
fn sevseg_three_digit_empty() {
    assert_eq!(sevseg_three(""), None);
}

#[test]
fn sevseg_four_digit_zero() {
    assert_eq!(
        sevseg_four("0000"),
        Some("┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━┓\n\
              ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛ ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_one() {
    assert_eq!(
        sevseg_four("1111"),
        Some("    ╻     ╻     ╻     ╻\n    ┃     ┃     ┃     ┃\n    ┃     ┃     ┃     \
            ┃\n    ┃     ┃     ┃     ┃\n    ╹     ╹     ╹     ╹\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_two() {
    assert_eq!(
        sevseg_four("2222"),
        Some("╺━━━┓ ╺━━━┓ ╺━━━┓ ╺━━━┓\n    ┃     ┃     ┃     ┃\n\
            ┏━━━┛ ┏━━━┛ ┏━━━┛ ┏━━━┛\n┃     ┃     ┃     ┃    \n┗━━━╸ ┗━━━╸ ┗━━━╸ ┗━━━╸\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_three() {
    assert_eq!(
        sevseg_four("3333"),
        Some("╺━━━┓ ╺━━━┓ ╺━━━┓ ╺━━━┓\n    ┃     ┃     ┃     ┃\n\
            ╺━━━┫ ╺━━━┫ ╺━━━┫ ╺━━━┫\n    ┃     ┃     ┃     ┃\n╺━━━┛ ╺━━━┛ ╺━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_four() {
    assert_eq!(
        sevseg_four("4444"),
        Some("╻   ╻ ╻   ╻ ╻   ╻ ╻   ╻\n┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
            ┗━━━┫ ┗━━━┫ ┗━━━┫ ┗━━━┫\n    ┃     ┃     ┃     ┃\n    ╹     ╹     ╹     ╹\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_five() {
    assert_eq!(
        sevseg_four("5555"),
        Some("┏━━━╸ ┏━━━╸ ┏━━━╸ ┏━━━╸\n┃     ┃     ┃     ┃    \n\
            ┗━━━┓ ┗━━━┓ ┗━━━┓ ┗━━━┓\n    ┃     ┃     ┃     ┃\n╺━━━┛ ╺━━━┛ ╺━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_six() {
    assert_eq!(
        sevseg_four("6666"),
        Some("┏━━━╸ ┏━━━╸ ┏━━━╸ ┏━━━╸\n\
              ┃     ┃     ┃     ┃    \n\
              ┣━━━┓ ┣━━━┓ ┣━━━┓ ┣━━━┓\n\
              ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛ ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_seven() {
    assert_eq!(
        sevseg_four("7777"),
        Some("╺━━━┓ ╺━━━┓ ╺━━━┓ ╺━━━┓\n    ┃     ┃     ┃     \
            ┃\n    ┃     ┃     ┃     ┃\n    ┃     ┃     ┃     ┃\n    ╹     ╹     ╹     ╹\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_eight() {
    assert_eq!(
        sevseg_four("8888"),
        Some("┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━┓\n\
              ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┣━━━┫ ┣━━━┫ ┣━━━┫ ┣━━━┫\n\
              ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
              ┗━━━┛ ┗━━━┛ ┗━━━┛ ┗━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_nine() {
    assert_eq!(
        sevseg_four("9999"),
        Some("┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━┓\n┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
            ┗━━━┫ ┗━━━┫ ┗━━━┫ ┗━━━┫\n    ┃     ┃     ┃     ┃\n╺━━━┛ ╺━━━┛ ╺━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_ten() {
    assert_eq!(
        sevseg_four("----"),
        Some("                       \n                       \n\
            ╺━━━╸ ╺━━━╸ ╺━━━╸ ╺━━━╸\n                       \n                       \n".to_string())
    );
}

#[test]
fn sevseg_four_digit_empty_ten() {
    assert_eq!(sevseg_four("-"),
        Some("┏━━━┓ ┏━━━┓ ┏━━━┓      \n\
              ┃   ┃ ┃   ┃ ┃   ┃      \n\
              ┃   ┃ ┃   ┃ ┃   ┃ ╺━━━╸\n\
              ┃   ┃ ┃   ┃ ┃   ┃      \n\
              ┗━━━┛ ┗━━━┛ ┗━━━┛      \n".to_string())
    );
}

#[test]
fn sevseg_four_digit_empty_five() {
        assert_eq!(sevseg_four("5"),
        Some("┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━╸\n\
              ┃   ┃ ┃   ┃ ┃   ┃ ┃    \n\
              ┃   ┃ ┃   ┃ ┃   ┃ ┗━━━┓\n\
              ┃   ┃ ┃   ┃ ┃   ┃     ┃\n\
              ┗━━━┛ ┗━━━┛ ┗━━━┛ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_empty_one_two() {
    assert_eq!(sevseg_four("12"),
        Some("┏━━━┓ ┏━━━┓     ╻ ╺━━━┓\n\
              ┃   ┃ ┃   ┃     ┃     ┃\n\
              ┃   ┃ ┃   ┃     ┃ ┏━━━┛\n\
              ┃   ┃ ┃   ┃     ┃ ┃    \n\
              ┗━━━┛ ┗━━━┛     ╹ ┗━━━╸\n".to_string())
   );
}

#[test]
fn sevseg_four_digit_empty_one_two_three() {
    assert_eq!(sevseg_four("125"),
        Some("┏━━━┓     ╻ ╺━━━┓ ┏━━━╸\n\
              ┃   ┃     ┃     ┃ ┃    \n\
              ┃   ┃     ┃ ┏━━━┛ ┗━━━┓\n\
              ┃   ┃     ┃ ┃         ┃\n\
              ┗━━━┛     ╹ ┗━━━╸ ╺━━━┛\n".to_string())
    );
}

#[test]
fn sevseg_four_digit_one_a_two() {
    assert_eq!(sevseg_four("1a23"), None);
}

#[test]
fn sevseg_four_digit_one_two_a() {
    assert_eq!(sevseg_four("123a"), None);
}

#[test]
fn sevseg_four_digit_a_none() {
    assert_eq!(sevseg_four("a123"), None);
}

#[test]
fn sevseg_four_digit_len_none() {
    assert_eq!(sevseg_four("01245"), None);
}

#[test]
fn sevseg_four_digit_two_none() {
    assert_eq!(sevseg_four("-----"), None);
}

#[test]
fn sevseg_four_digit_empty() {
    assert_eq!(sevseg_four(""), None);
}

#[test]
fn sevseg_four_iter_digit_zero() {
    assert_eq!(
        &sevseg_four_iter("0000").unwrap().collect::<String>(),
        "┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━┓\n\
         ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
         ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
         ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
         ┗━━━┛ ┗━━━┛ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_four_iter_digit_one() {
    assert_eq!(
        &sevseg_four_iter("1111").unwrap().collect::<String>(),
        "    ╻     ╻     ╻     ╻\n    ┃     ┃     ┃     ┃\n    ┃     ┃     ┃     \
            ┃\n    ┃     ┃     ┃     ┃\n    ╹     ╹     ╹     ╹\n"
    );
}

#[test]
fn sevseg_four_iter_digit_two() {
    assert_eq!(
        &sevseg_four_iter("2222").unwrap().collect::<String>(),
        "╺━━━┓ ╺━━━┓ ╺━━━┓ ╺━━━┓\n    ┃     ┃     ┃     ┃\n\
            ┏━━━┛ ┏━━━┛ ┏━━━┛ ┏━━━┛\n┃     ┃     ┃     ┃    \n┗━━━╸ ┗━━━╸ ┗━━━╸ ┗━━━╸\n"
    );
}

#[test]
fn sevseg_four_iter_digit_three() {
    assert_eq!(
        &sevseg_four_iter("3333").unwrap().collect::<String>(),
        "╺━━━┓ ╺━━━┓ ╺━━━┓ ╺━━━┓\n    ┃     ┃     ┃     ┃\n\
            ╺━━━┫ ╺━━━┫ ╺━━━┫ ╺━━━┫\n    ┃     ┃     ┃     ┃\n╺━━━┛ ╺━━━┛ ╺━━━┛ ╺━━━┛\n"
    );
}

#[test]
fn sevseg_four_iter_digit_four() {
    assert_eq!(
        &sevseg_four_iter("4444").unwrap().collect::<String>(),
        "╻   ╻ ╻   ╻ ╻   ╻ ╻   ╻\n┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
            ┗━━━┫ ┗━━━┫ ┗━━━┫ ┗━━━┫\n    ┃     ┃     ┃     ┃\n    ╹     ╹     ╹     ╹\n"
    );
}

#[test]
fn sevseg_four_iter_digit_five() {
    assert_eq!(
        &sevseg_four_iter("5555").unwrap().collect::<String>(),
        "┏━━━╸ ┏━━━╸ ┏━━━╸ ┏━━━╸\n┃     ┃     ┃     ┃    \n\
            ┗━━━┓ ┗━━━┓ ┗━━━┓ ┗━━━┓\n    ┃     ┃     ┃     ┃\n╺━━━┛ ╺━━━┛ ╺━━━┛ ╺━━━┛\n"
    );
}

#[test]
fn sevseg_four_iter_digit_six() {
    assert_eq!(
        &sevseg_four_iter("6666").unwrap().collect::<String>(),
        "┏━━━╸ ┏━━━╸ ┏━━━╸ ┏━━━╸\n\
         ┃     ┃     ┃     ┃    \n\
         ┣━━━┓ ┣━━━┓ ┣━━━┓ ┣━━━┓\n\
         ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
         ┗━━━┛ ┗━━━┛ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_four_iter_digit_seven() {
    assert_eq!(
        &sevseg_four_iter("7777").unwrap().collect::<String>(),
        "╺━━━┓ ╺━━━┓ ╺━━━┓ ╺━━━┓\n    ┃     ┃     ┃     \
            ┃\n    ┃     ┃     ┃     ┃\n    ┃     ┃     ┃     ┃\n    ╹     ╹     ╹     ╹\n"
    );
}

#[test]
fn sevseg_four_iter_digit_eight() {
    assert_eq!(
        &sevseg_four_iter("8888").unwrap().collect::<String>(),
        "┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━┓\n\
         ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
         ┣━━━┫ ┣━━━┫ ┣━━━┫ ┣━━━┫\n\
         ┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
         ┗━━━┛ ┗━━━┛ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_four_iter_digit_nine() {
    assert_eq!(
        &sevseg_four_iter("9999").unwrap().collect::<String>(),
        "┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━┓\n┃   ┃ ┃   ┃ ┃   ┃ ┃   ┃\n\
            ┗━━━┫ ┗━━━┫ ┗━━━┫ ┗━━━┫\n    ┃     ┃     ┃     ┃\n╺━━━┛ ╺━━━┛ ╺━━━┛ ╺━━━┛\n"
    );
}

#[test]
fn sevseg_four_iter_digit_ten() {
    assert_eq!(
        &sevseg_four_iter("----").unwrap().collect::<String>(),
        "                       \n                       \n\
            ╺━━━╸ ╺━━━╸ ╺━━━╸ ╺━━━╸\n                       \n                       \n"
    );
}

#[test]
fn sevseg_four_iter_digit_empty_ten() {
    assert_eq!(&sevseg_four_iter("-").unwrap().collect::<String>(),
        "┏━━━┓ ┏━━━┓ ┏━━━┓      \n\
         ┃   ┃ ┃   ┃ ┃   ┃      \n\
         ┃   ┃ ┃   ┃ ┃   ┃ ╺━━━╸\n\
         ┃   ┃ ┃   ┃ ┃   ┃      \n\
         ┗━━━┛ ┗━━━┛ ┗━━━┛      \n"
    );
}

#[test]
fn sevseg_four_iter_digit_empty_five() {
    assert_eq!(&sevseg_four_iter("5").unwrap().collect::<String>(),
        "┏━━━┓ ┏━━━┓ ┏━━━┓ ┏━━━╸\n\
         ┃   ┃ ┃   ┃ ┃   ┃ ┃    \n\
         ┃   ┃ ┃   ┃ ┃   ┃ ┗━━━┓\n\
         ┃   ┃ ┃   ┃ ┃   ┃     ┃\n\
         ┗━━━┛ ┗━━━┛ ┗━━━┛ ╺━━━┛\n"
    );
}

#[test]
fn sevseg_four_iter_digit_empty_one_two() {
    assert_eq!(&sevseg_four_iter("12").unwrap().collect::<String>(),
        "┏━━━┓ ┏━━━┓     ╻ ╺━━━┓\n\
         ┃   ┃ ┃   ┃     ┃     ┃\n\
         ┃   ┃ ┃   ┃     ┃ ┏━━━┛\n\
         ┃   ┃ ┃   ┃     ┃ ┃    \n\
         ┗━━━┛ ┗━━━┛     ╹ ┗━━━╸\n"
    );
}

#[test]
fn sevseg_four_iter_digit_empty_one_two_three() {
    assert_eq!(&sevseg_four_iter("125").unwrap().collect::<String>(),
        "┏━━━┓     ╻ ╺━━━┓ ┏━━━╸\n\
         ┃   ┃     ┃     ┃ ┃    \n\
         ┃   ┃     ┃ ┏━━━┛ ┗━━━┓\n\
         ┃   ┃     ┃ ┃         ┃\n\
         ┗━━━┛     ╹ ┗━━━╸ ╺━━━┛\n"
    );
}

#[test]
fn sevseg_four_iter_digit_one_a_two() {
    assert!(sevseg_four_iter("1a23").is_none());
}

#[test]
fn sevseg_four_iter_digit_one_two_a() {
    assert!(sevseg_four_iter("123a").is_none());
}

#[test]
fn sevseg_four_iter_digit_a_none() {
    assert!(sevseg_four_iter("a123").is_none());
}

#[test]
fn sevseg_four_iter_digit_len_none() {
    assert!(sevseg_four_iter("01245").is_none());
}

#[test]
fn sevseg_four_iter_digit_two_none() {
    assert!(sevseg_four_iter("-----").is_none());
}

#[test]
fn sevseg_four_iter_digit_empty() {
    assert!(sevseg_four_iter("").is_none());
}
