use std::io;

use crate::models::PasswordOption;
use crate::utils::confirmation_user;
use crate::services::{build_charset, generate_password};

pub fn password_gen() {
    println!("Input password length: ");
    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read input");

    if input_number.trim().is_empty() {
        println!("Program exit, no value given.");
        return;
    }

    let number: usize = match input_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: input must be a number");
            return;
        }
    };

    if number < 4 {
        println!("Error: minimum length is 4");
        return;
    }

    let use_uppercase = confirmation_user("Include uppercase letters?");
    let use_digits = confirmation_user("Include digtis?");
    let use_symbols = confirmation_user("Include symbols?");

    let options = PasswordOption {
        length: number,
        use_uppercase,
        use_digits,
        use_symbols,
    };

    let charset = build_charset(&options);
    let password = generate_password(number, &charset);

    options.debug_print();
    println!("Generated Password: {password}");
}