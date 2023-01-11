#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    let trimmed_number = cc_number.replace(" ", "");
    let digits: Vec<u32> = trimmed_number
        .chars()
        .filter_map(|char| char.to_digit(10))
        .collect();
    if digits.len() < 2 {
        return false;
    }
    let mut sum = 0;
    let length = digits.len();
    let parity = (length + 1) % 2;
    for index in 0..length {
        let digit = digits[index];
        if index % 2 == parity {
            sum += digit;
        } else if digit > 4 {
            sum += 2 * digit - 9
        } else {
            sum += 2 * digit
        }
    }
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
