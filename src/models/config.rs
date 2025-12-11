use serde::{Deserialize, Serialize};

use crate::models::PasswordOption;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub length: usize,
    pub uppercase: bool,
    pub digits: bool,
    pub symbols: bool,
}

impl Config {
    pub fn password_options(&self) -> PasswordOption {
        PasswordOption {
            length: self.length,
            use_uppercase: self.uppercase,
            use_digits: self.digits,
            use_symbols: self.symbols,
        }
    }
}
