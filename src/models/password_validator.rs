pub struct PasswordValidatorResult {
    pub is_valid: bool,
    pub valid_min_length: bool,
    pub valid_lowercase: bool,
    pub valid_uppercase: bool,
    pub valid_digits: bool,
    pub valid_symbols: bool,
}
