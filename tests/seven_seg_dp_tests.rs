use seven_seg::sevseg_four_dp;

#[test]
fn sevseg_dp_neg_point_pos_three_zero() {
    let val_dp: f32 = -1.0;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻  ┏━━━┓ ┏━━━┓ ┏━━━┓\n      ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ━━┃  ┃   ┃ ┃   ┃ ┃   ┃\n      ┃  ┃   ┃ ┃   ┃ ┃   ┃\n      ╹⦁ ┗━━━┛ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_neg_point_pos_four_zero() {
    let val_dp: f32 = -12.0;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓  ┏━━━┓ ┏━━━┓\n      ┃     ┃  ┃   ┃ ┃   ┃\n    ━━┃ ┏━━━┛  ┃   ┃ ┃   ┃\n      ┃ ┃      ┃   ┃ ┃   ┃\n      ╹ ┗━━━╸⦁ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_neg_point_pos_five_zero() {
    let val_dp: f32 = -123.0;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓ ╺━━━┓  ┏━━━┓\n      ┃     ┃     ┃  ┃   ┃\n    ━━┃ ┏━━━┛ ╺━━━┫  ┃   ┃\n      ┃ ┃         ┃  ┃   ┃\n      ╹ ┗━━━╸ ╺━━━┛⦁ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_neg_point_pos_six_zero() {
    let val_dp: f64 = -1234.0;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓ ╺━━━┓ ╻   ╻ \n      ┃     ┃     ┃ ┃   ┃ \n    ━━┃ ┏━━━┛ ╺━━━┫ ┗━━━┫ \n      ┃ ┃         ┃     ┃ \n      ╹ ┗━━━╸ ╺━━━┛     ╹⦁\n"
    );
}

#[test]
fn sevseg_dp_point_pos_seven() {
    let val_dp: f64 = -123456.789;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓ ╺━━━┓ ╻   ╻\n      ┃     ┃     ┃ ┃   ┃\n    ━━┃ ┏━━━┛ ╺━━━┫ ┗━━━┫\n      ┃ ┃         ┃     ┃\n      ╹ ┗━━━╸ ╺━━━┛     ╹\n"
    );
}

#[test]
fn sevseg_dp_neg_len_one_empty() {
    let val_dp: i8 = -1;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻  ┏━━━┓ ┏━━━┓ ┏━━━┓\n      ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ━━┃  ┃   ┃ ┃   ┃ ┃   ┃\n      ┃  ┃   ┃ ┃   ┃ ┃   ┃\n      ╹⦁ ┗━━━┛ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_len_one_empty() {
    let val_dp: u8 = 1;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻  ┏━━━┓ ┏━━━┓ ┏━━━┓\n    ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ╹⦁ ┗━━━┛ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_neg_len_two_empty() {
    let val_dp: i8 = -12;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓  ┏━━━┓ ┏━━━┓\n      ┃     ┃  ┃   ┃ ┃   ┃\n    ━━┃ ┏━━━┛  ┃   ┃ ┃   ┃\n      ┃ ┃      ┃   ┃ ┃   ┃\n      ╹ ┗━━━╸⦁ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_len_two_empty() {
    let val_dp: u8 = 12;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻ ╺━━━┓  ┏━━━┓ ┏━━━┓\n    ┃     ┃  ┃   ┃ ┃   ┃\n    ┃ ┏━━━┛  ┃   ┃ ┃   ┃\n    ┃ ┃      ┃   ┃ ┃   ┃\n    ╹ ┗━━━╸⦁ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_neg_len_three_empty() {
    let val_dp: i8 = -123;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓ ╺━━━┓  ┏━━━┓\n      ┃     ┃     ┃  ┃   ┃\n    ━━┃ ┏━━━┛ ╺━━━┫  ┃   ┃\n      ┃ ┃         ┃  ┃   ┃\n      ╹ ┗━━━╸ ╺━━━┛⦁ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_len_three_empty() {
    let val_dp: u8 = 123;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻ ╺━━━┓ ╺━━━┓  ┏━━━┓\n    ┃     ┃     ┃  ┃   ┃\n    ┃ ┏━━━┛ ╺━━━┫  ┃   ┃\n    ┃ ┃         ┃  ┃   ┃\n    ╹ ┗━━━╸ ╺━━━┛⦁ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_neg_len_four_empty() {
    let val_dp: i16 = -1234;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓ ╺━━━┓ ╻   ╻ \n      ┃     ┃     ┃ ┃   ┃ \n    ━━┃ ┏━━━┛ ╺━━━┫ ┗━━━┫ \n      ┃ ┃         ┃     ┃ \n      ╹ ┗━━━╸ ╺━━━┛     ╹⦁\n"
    );
}

#[test]
fn sevseg_dp_len_four_empty() {
    let val_dp: u16 = 1234;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻ ╺━━━┓ ╺━━━┓ ╻   ╻ \n    ┃     ┃     ┃ ┃   ┃ \n    ┃ ┏━━━┛ ╺━━━┫ ┗━━━┫ \n    ┃ ┃         ┃     ┃ \n    ╹ ┗━━━╸ ╺━━━┛     ╹⦁\n"
    );
}

#[test]
fn sevseg_dp_neg_len_five() {
    let val_dp: i16 = -12345;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓ ╺━━━┓ ╻   ╻\n      ┃     ┃     ┃ ┃   ┃\n    ━━┃ ┏━━━┛ ╺━━━┫ ┗━━━┫\n      ┃ ┃         ┃     ┃\n      ╹ ┗━━━╸ ╺━━━┛     ╹\n"
    );
}

#[test]
fn sevseg_dp_len_five() {
    let val_dp: u16 = 12345;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻ ╺━━━┓ ╺━━━┓ ╻   ╻\n    ┃     ┃     ┃ ┃   ┃\n    ┃ ┏━━━┛ ╺━━━┫ ┗━━━┫\n    ┃ ┃         ┃     ┃\n    ╹ ┗━━━╸ ╺━━━┛     ╹\n"
    );
}

#[test]
fn sevseg_neg_len_nine() {
    let val_dp: i32 = -123456789;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓ ╺━━━┓ ╻   ╻\n      ┃     ┃     ┃ ┃   ┃\n    ━━┃ ┏━━━┛ ╺━━━┫ ┗━━━┫\n      ┃ ┃         ┃     ┃\n      ╹ ┗━━━╸ ╺━━━┛     ╹\n"
    );
}

#[test]
fn sevseg_dp_neg_point_pos_three_empty() {
    let val_dp: f32 = -1.;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻  ┏━━━┓ ┏━━━┓ ┏━━━┓\n      ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ━━┃  ┃   ┃ ┃   ┃ ┃   ┃\n      ┃  ┃   ┃ ┃   ┃ ┃   ┃\n      ╹⦁ ┗━━━┛ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_neg_point_pos_four_empty() {
    let val_dp: f32 = -12.;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓  ┏━━━┓ ┏━━━┓\n      ┃     ┃  ┃   ┃ ┃   ┃\n    ━━┃ ┏━━━┛  ┃   ┃ ┃   ┃\n      ┃ ┃      ┃   ┃ ┃   ┃\n      ╹ ┗━━━╸⦁ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_neg_point_pos_five_empty() {
    let val_dp: f32 = -123.;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓ ╺━━━┓  ┏━━━┓\n      ┃     ┃     ┃  ┃   ┃\n    ━━┃ ┏━━━┛ ╺━━━┫  ┃   ┃\n      ┃ ┃         ┃  ┃   ┃\n      ╹ ┗━━━╸ ╺━━━┛⦁ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_neg_point_pos_six_empty() {
    let val_dp: f32 = -1234.;
    assert_eq!(sevseg_four_dp(val_dp),
    "      ╻ ╺━━━┓ ╺━━━┓ ╻   ╻ \n      ┃     ┃     ┃ ┃   ┃ \n    ━━┃ ┏━━━┛ ╺━━━┫ ┗━━━┫ \n      ┃ ┃         ┃     ┃ \n      ╹ ┗━━━╸ ╺━━━┛     ╹⦁\n"
    );
}

#[test]
fn sevseg_dp_point_pos_two_zero() {
    let val_dp: f32 = 1.0;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻  ┏━━━┓ ┏━━━┓ ┏━━━┓\n    ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ╹⦁ ┗━━━┛ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_point_pos_three_zero() {
    let val_dp: f32 = 12.0;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻ ╺━━━┓  ┏━━━┓ ┏━━━┓\n    ┃     ┃  ┃   ┃ ┃   ┃\n    ┃ ┏━━━┛  ┃   ┃ ┃   ┃\n    ┃ ┃      ┃   ┃ ┃   ┃\n    ╹ ┗━━━╸⦁ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_point_pos_four_zero() {
    let val_dp: f32 = 123.0;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻ ╺━━━┓ ╺━━━┓  ┏━━━┓\n    ┃     ┃     ┃  ┃   ┃\n    ┃ ┏━━━┛ ╺━━━┫  ┃   ┃\n    ┃ ┃         ┃  ┃   ┃\n    ╹ ┗━━━╸ ╺━━━┛⦁ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_point_pos_five_zero() {
    let val_dp: f32 = 1234.0;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻ ╺━━━┓ ╺━━━┓ ╻   ╻ \n    ┃     ┃     ┃ ┃   ┃ \n    ┃ ┏━━━┛ ╺━━━┫ ┗━━━┫ \n    ┃ ┃         ┃     ┃ \n    ╹ ┗━━━╸ ╺━━━┛     ╹⦁\n"
    );
}

#[test]
fn sevseg_dp_point_pos_six_zero() {
    let val_dp: f32 = 12345.0;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻ ╺━━━┓ ╺━━━┓ ╻   ╻\n    ┃     ┃     ┃ ┃   ┃\n    ┃ ┏━━━┛ ╺━━━┫ ┗━━━┫\n    ┃ ┃         ┃     ┃\n    ╹ ┗━━━╸ ╺━━━┛     ╹\n"
    );
}

#[test]
fn sevseg_dp_point_pos_three() {
    let val_dp: f32 = -5.1000;
    assert_eq!(sevseg_four_dp(val_dp),
    "  ┏━━━╸      ╻ ┏━━━┓ ┏━━━┓\n  ┃          ┃ ┃   ┃ ┃   ┃\n\
    ━━┗━━━┓      ┃ ┃   ┃ ┃   ┃\n      ┃      ┃ ┃   ┃ ┃   ┃\n  ╺━━━┛⦁     ╹ ┗━━━┛ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_point_pos_four() {
    let val_dp: f32 = -51.693;
    assert_eq!(sevseg_four_dp(val_dp),
    "  ┏━━━╸     ╻  ┏━━━╸ ┏━━━┓\n  ┃         ┃  ┃     ┃   ┃\n\
    ━━┗━━━┓     ┃  ┣━━━┓ ┗━━━┫\n      ┃     ┃  ┃   ┃     ┃\n  ╺━━━┛     ╹⦁ ┗━━━┛ ╺━━━┛\n"
    );
}

#[test]
fn sevseg_dp_point_pos_five() {
    let val_dp: f32 = -516.93;
    assert_eq!(sevseg_four_dp(val_dp),
    "  ┏━━━╸     ╻ ┏━━━╸  ┏━━━┓\n  ┃         ┃ ┃      ┃   ┃\n\
    ━━┗━━━┓     ┃ ┣━━━┓  ┗━━━┫\n      ┃     ┃ ┃   ┃      ┃\n  ╺━━━┛     ╹ ┗━━━┛⦁ ╺━━━┛\n"
    );
}

#[test]
fn sevseg_dp_point_pos_six() {
    let val_dp: f32 = -5169.3;
    assert_eq!(sevseg_four_dp(val_dp),
    "  ┏━━━╸     ╻ ┏━━━╸ ┏━━━┓ \n  ┃         ┃ ┃     ┃   ┃ \n\
    ━━┗━━━┓     ┃ ┣━━━┓ ┗━━━┫ \n      ┃     ┃ ┃   ┃     ┃ \n  ╺━━━┛     ╹ ┗━━━┛ ╺━━━┛⦁\n"
    );
}

#[test]
fn sevseg_dp_point_pos_two_no_digits() {
    let val_dp: f32 = 1.;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻  ┏━━━┓ ┏━━━┓ ┏━━━┓\n    ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ┃  ┃   ┃ ┃   ┃ ┃   ┃\n    ╹⦁ ┗━━━┛ ┗━━━┛ ┗━━━┛\n"
    );
}


#[test]
fn sevseg_dp_point_pos_two_positive() {
    let val_dp: f64 = 1.23456789;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻  ╺━━━┓ ╺━━━┓ ╻   ╻\n    ┃      ┃     ┃ ┃   ┃\n    ┃  ┏━━━┛ ╺━━━┫ ┗━━━┫\n    ┃  ┃         ┃     ┃\n    ╹⦁ ┗━━━╸ ╺━━━┛     ╹\n"
    );
}

#[test]
fn sevseg_dp_point_pos_three_psitive() {
    let val_dp: f64 = 12.3456789;
    assert_eq!(sevseg_four_dp(val_dp),
    "    ╻ ╺━━━┓  ╺━━━┓ ╻   ╻\n    ┃     ┃      ┃ ┃   ┃\n    ┃ ┏━━━┛  ╺━━━┫ ┗━━━┫\n    ┃ ┃          ┃     ┃\n    ╹ ┗━━━╸⦁ ╺━━━┛     ╹\n"
    );
}

#[test]
fn sevseg_dp_point_pos_four_psitive() {
let val_dp: f64 = 562.0421;
    assert_eq!(sevseg_four_dp(val_dp),
    "┏━━━╸ ┏━━━╸ ╺━━━┓  ┏━━━┓\n┃     ┃         ┃  ┃   ┃\n┗━━━┓ ┣━━━┓ ┏━━━┛  ┃   ┃\n    ┃ ┃   ┃ ┃      ┃   ┃\n╺━━━┛ ┗━━━┛ ┗━━━╸⦁ ┗━━━┛\n"
    );
}

#[test]
fn sevseg_dp_point_pos_six_psitive() {
let val_dp: f64 = 5620.421;
    assert_eq!(sevseg_four_dp(val_dp),
    "┏━━━╸ ┏━━━╸ ╺━━━┓ ┏━━━┓ \n┃     ┃         ┃ ┃   ┃ \n┗━━━┓ ┣━━━┓ ┏━━━┛ ┃   ┃ \n    ┃ ┃   ┃ ┃     ┃   ┃ \n╺━━━┛ ┗━━━┛ ┗━━━╸ ┗━━━┛⦁\n"
    );
}

#[test]
fn sevseg_dp_point_pos_seven_psitive() {
let val_dp: f64 = 56204.21;
    assert_eq!(sevseg_four_dp(val_dp),
    "┏━━━╸ ┏━━━╸ ╺━━━┓ ┏━━━┓\n┃     ┃         ┃ ┃   ┃\n┗━━━┓ ┣━━━┓ ┏━━━┛ ┃   ┃\n    ┃ ┃   ┃ ┃     ┃   ┃\n╺━━━┛ ┗━━━┛ ┗━━━╸ ┗━━━┛\n"
    );
}
