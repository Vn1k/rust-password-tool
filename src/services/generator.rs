use crate::config::{DIGITS, LOWERCASE, SYMBOLS, UPPERCASE};
use crate::models::PasswordOption;
use rand::{Rng, rngs::OsRng};

pub fn build_charset(options: &PasswordOption) -> Vec<u8> {
    let mut charset = Vec::new();

    charset.extend_from_slice(LOWERCASE);

    if options.use_uppercase {
        charset.extend_from_slice(UPPERCASE);
    }

    if options.use_digits {
        charset.extend_from_slice(DIGITS);
    }

    if options.use_symbols {
        charset.extend_from_slice(SYMBOLS);
    }

    charset
}

pub fn generate_password(
    length: usize,
    charset: &[u8],
) -> String {
    let mut rng = OsRng;
    let mut password = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.gen_range(0..charset.len()); //get random from 0 to charset length
        let byte = charset[idx]; //get the index of charset 
        let ch = byte as char; //convert it to charachter
        password.push(ch); //converted byte store to password variable
    }

    password
}
