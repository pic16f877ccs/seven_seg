#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]
//! Pseudo seven segment digital display.
//!
//! One, two, three and four digits.
//! One digit, it can be a string of numbers from zero to nine or a dash no value.
//!
//! # Examples
//!
//! ```
//! # use seven_seg::sevseg_four;
//! let four_digits = sevseg_four("8023").unwrap();
//!
//! assert_eq!(&four_digits, "┏━━━┓ ┏━━━┓ ╺━━━┓ ╺━━━┓\n\
//!                           ┃   ┃ ┃   ┃     ┃     ┃\n\
//!                           ┣━━━┫ ┃   ┃ ┏━━━┛ ╺━━━┫\n\
//!                           ┃   ┃ ┃   ┃ ┃         ┃\n\
//!                           ┗━━━┛ ┗━━━┛ ┗━━━╸ ╺━━━┛\n"
//! );
//! ```
use cattocol::{by_four_lines, by_lines, by_three_lines};

const SEVEN_SEG: [&str; 11] = [
    "┏━━━┓\n┃   ┃\n┃   ┃\n┃   ┃\n┗━━━┛\n",
    "    ╻\n    ┃\n    ┃\n    ┃\n    ╹\n",
    "╺━━━┓\n    ┃\n┏━━━┛\n┃    \n┗━━━╸\n",
    "╺━━━┓\n    ┃\n╺━━━┫\n    ┃\n╺━━━┛\n",
    "╻   ╻\n┃   ┃\n┗━━━┫\n    ┃\n    ╹\n",
    "┏━━━╸\n┃    \n┗━━━┓\n    ┃\n╺━━━┛\n",
    "┏━━━╸\n┃    \n┣━━━┓\n┃   ┃\n┗━━━┛\n",
    "╺━━━┓\n    ┃\n    ┃\n    ┃\n    ╹\n",
    "┏━━━┓\n┃   ┃\n┣━━━┫\n┃   ┃\n┗━━━┛\n",
    "┏━━━┓\n┃   ┃\n┗━━━┫\n    ┃\n╺━━━┛\n",
    "     \n     \n╺━━━╸\n     \n     \n",
];

/// One digits seven segment digital display.
///
/// # Examples
///
/// ```
/// # use seven_seg::sevseg_one;
/// let one_digits = sevseg_one("8").unwrap();
///
/// assert_eq!(&one_digits, "┏━━━┓\n\
///                          ┃   ┃\n\
///                          ┣━━━┫\n\
///                          ┃   ┃\n\
///                          ┗━━━┛\n"
/// );
/// ```
#[inline]
pub fn sevseg_one(string: &str) -> Option<String> {
    let digit = if 1 == string.chars().count() {
        if let Ok(digit) = string.parse() {
            digit
        } else if string == "-" {
            10
        } else {
            return None;
        }
    } else {
        return None;
    };

    Some(SEVEN_SEG[digit].to_string())
}

/// Two digits seven segment digital display.
///
/// # Examples
///
/// ```
/// # use seven_seg::sevseg_two;
/// let two_digits = sevseg_two("80").unwrap();
///
/// assert_eq!(&two_digits, "┏━━━┓ ┏━━━┓\n\
///                          ┃   ┃ ┃   ┃\n\
///                          ┣━━━┫ ┃   ┃\n\
///                          ┃   ┃ ┃   ┃\n\
///                          ┗━━━┛ ┗━━━┛\n"
/// );
/// ```
///
/// ```
/// # use seven_seg::sevseg_two;
/// let two_digits = sevseg_two("-").unwrap();
///
/// assert_eq!(&two_digits, "┏━━━┓      \n\
///                          ┃   ┃      \n\
///                          ┃   ┃ ╺━━━╸\n\
///                          ┃   ┃      \n\
///                          ┗━━━┛      \n"
/// );
/// ```
#[inline]
pub fn sevseg_two(string: &str) -> Option<String> {
    let len = string.chars().count();
    let first_digit = if len >= 1 { &string[0..1] } else { "" };
    let second_digit = if len == 2 { &string[1..2] } else { "" };

    let digit: (usize, usize) = if 2 == len {
        (
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = second_digit.parse() {
                digit
            } else if second_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else if 1 == len {
        (
            0,
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else {
        return None;
    };

    Some(by_lines(SEVEN_SEG[digit.0], SEVEN_SEG[digit.1]).collect::<String>())
}

/// Three digits seven segment digital display.
///
/// # Examples
///
/// ```
/// # use seven_seg::sevseg_three;
/// let three_digits = sevseg_three("802").unwrap();
///
/// assert_eq!(&three_digits, "┏━━━┓ ┏━━━┓ ╺━━━┓\n\
///                            ┃   ┃ ┃   ┃     ┃\n\
///                            ┣━━━┫ ┃   ┃ ┏━━━┛\n\
///                            ┃   ┃ ┃   ┃ ┃    \n\
///                            ┗━━━┛ ┗━━━┛ ┗━━━╸\n"
/// );
/// ```
///
/// ```
/// # use seven_seg::sevseg_three;
/// let three_digits = sevseg_three("2").unwrap();
///
/// assert_eq!(&three_digits, "┏━━━┓ ┏━━━┓ ╺━━━┓\n\
///                            ┃   ┃ ┃   ┃     ┃\n\
///                            ┃   ┃ ┃   ┃ ┏━━━┛\n\
///                            ┃   ┃ ┃   ┃ ┃    \n\
///                            ┗━━━┛ ┗━━━┛ ┗━━━╸\n"
/// );
/// ```
pub fn sevseg_three(string: &str) -> Option<String> {
    let len = string.chars().count();
    let first_digit = if len >= 1 { &string[0..1] } else { "" };
    let second_digit = if len >= 2 { &string[1..2] } else { "" };
    let third_digit = if len == 3 { &string[2..3] } else { "" };

    let digit: (usize, usize, usize) = if 3 == len {
        (
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = second_digit.parse() {
                digit
            } else if second_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = third_digit.parse() {
                digit
            } else if third_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else if 2 == len {
        (
            0,
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = second_digit.parse() {
                digit
            } else if second_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else if 1 == len {
        (
            0,
            0,
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else {
        return None;
    };

    Some(
        by_three_lines(SEVEN_SEG[digit.0], SEVEN_SEG[digit.1], SEVEN_SEG[digit.2])
            .collect::<String>(),
    )
}

/// Four digits seven segment digital display.
///
/// # Examples
///
/// ```
/// # use seven_seg::sevseg_four;
/// let four_digits = sevseg_four("8023").unwrap();
///
/// assert_eq!(&four_digits, "┏━━━┓ ┏━━━┓ ╺━━━┓ ╺━━━┓\n\
///                           ┃   ┃ ┃   ┃     ┃     ┃\n\
///                           ┣━━━┫ ┃   ┃ ┏━━━┛ ╺━━━┫\n\
///                           ┃   ┃ ┃   ┃ ┃         ┃\n\
///                           ┗━━━┛ ┗━━━┛ ┗━━━╸ ╺━━━┛\n"
/// );
/// ```
///
/// ```
/// # use seven_seg::sevseg_four;
/// let four_digits = sevseg_four("023").unwrap();
///
/// assert_eq!(&four_digits, "┏━━━┓ ┏━━━┓ ╺━━━┓ ╺━━━┓\n\
///                           ┃   ┃ ┃   ┃     ┃     ┃\n\
///                           ┃   ┃ ┃   ┃ ┏━━━┛ ╺━━━┫\n\
///                           ┃   ┃ ┃   ┃ ┃         ┃\n\
///                           ┗━━━┛ ┗━━━┛ ┗━━━╸ ╺━━━┛\n"
/// );
/// ```
pub fn sevseg_four(string: &str) -> Option<String> {
    let len = string.chars().count();
    let first_digit = if len >= 1 { &string[0..1] } else { "" };
    let second_digit = if len >= 2 { &string[1..2] } else { "" };
    let third_digit = if len >= 3 { &string[2..3] } else { "" };
    let fourth_digit = if len == 4 { &string[3..4] } else { "" };

    let digit: (usize, usize, usize, usize) = if 4 == len {
        (
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = second_digit.parse() {
                digit
            } else if second_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = third_digit.parse() {
                digit
            } else if third_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = fourth_digit.parse() {
                digit
            } else if fourth_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else if 3 == len {
        (
            0,
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = second_digit.parse() {
                digit
            } else if second_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = third_digit.parse() {
                digit
            } else if third_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else if 2 == len {
        (
            0,
            0,
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = second_digit.parse() {
                digit
            } else if second_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else if 1 == len {
        (
            0,
            0,
            0,
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else {
        return None;
    };

    Some(
        by_four_lines(
            SEVEN_SEG[digit.0],
            SEVEN_SEG[digit.1],
            SEVEN_SEG[digit.2],
            SEVEN_SEG[digit.3],
        )
        .collect::<String>(),
    )
}

/// Four digits seven segment digital display returns an iterator.
///
/// # Examples
///
/// ```
/// # use seven_seg::sevseg_four_iter;
/// let four_digits = sevseg_four_iter("8023").unwrap().collect::<String>();
///
/// assert_eq!(&four_digits, "┏━━━┓ ┏━━━┓ ╺━━━┓ ╺━━━┓\n\
///                           ┃   ┃ ┃   ┃     ┃     ┃\n\
///                           ┣━━━┫ ┃   ┃ ┏━━━┛ ╺━━━┫\n\
///                           ┃   ┃ ┃   ┃ ┃         ┃\n\
///                           ┗━━━┛ ┗━━━┛ ┗━━━╸ ╺━━━┛\n"
/// );
/// ```
pub fn sevseg_four_iter(string: &str) -> Option<impl Iterator<Item = &str>> {
    let len = string.chars().count();
    let first_digit = if len >= 1 { &string[0..1] } else { "" };
    let second_digit = if len >= 2 { &string[1..2] } else { "" };
    let third_digit = if len >= 3 { &string[2..3] } else { "" };
    let fourth_digit = if len == 4 { &string[3..4] } else { "" };

    let digit: (usize, usize, usize, usize) = if 4 == len {
        (
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = second_digit.parse() {
                digit
            } else if second_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = third_digit.parse() {
                digit
            } else if third_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = fourth_digit.parse() {
                digit
            } else if fourth_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else if 3 == len {
        (
            0,
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = second_digit.parse() {
                digit
            } else if second_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = third_digit.parse() {
                digit
            } else if third_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else if 2 == len {
        (
            0,
            0,
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
            if let Ok(digit) = second_digit.parse() {
                digit
            } else if second_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else if 1 == len {
        (
            0,
            0,
            0,
            if let Ok(digit) = first_digit.parse() {
                digit
            } else if first_digit == "-" {
                10
            } else {
                return None;
            },
        )
    } else {
        return None;
    };

    Some(by_four_lines(
        SEVEN_SEG[digit.0],
        SEVEN_SEG[digit.1],
        SEVEN_SEG[digit.2],
        SEVEN_SEG[digit.3],
    ))
}

