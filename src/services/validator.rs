use crate::models::PasswordValidatorResult;

pub fn validate_password(password: &str) -> PasswordValidatorResult {
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_digits = false;
    let mut has_symbols = false;

    for ch in password.chars() {
        if ch.is_ascii_lowercase() {
            has_lowercase = true;
        } else if ch.is_ascii_uppercase() {
            has_uppercase = true;
        } else if ch.is_ascii_digit() {
            has_digits = true;
        } else {
            has_symbols = true;
        }
    }

    let mut category_count = 0;
    if has_lowercase {
        category_count += 1;
    }
    if has_uppercase {
        category_count += 1;
    }
    if has_digits {
        category_count += 1;
    }
    if has_symbols {
        category_count += 1;
    }

    let valid_category = category_count >= 3;
    let valid_min_length = password.len() >= 12;
    let valid_lowercase = has_lowercase;
    let valid_uppercase = has_uppercase;
    let valid_digits = has_digits;
    let valid_symbols = has_symbols;

    let is_valid = valid_min_length && valid_lowercase && valid_category;

    let validator = PasswordValidatorResult {
        is_valid,
        valid_min_length,
        valid_lowercase,
        valid_uppercase,
        valid_digits,
        valid_symbols,
    };

    validator
}
