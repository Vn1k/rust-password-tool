use std::io;

use crate::services::validate_password;

pub fn password_check() {
    println!("Enter password to validate:");

    let mut password = String::new();

    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read input");

    if password.trim().is_empty() {
        println!("No input provided.");
        return;
    }

    let trimmed = password.trim().to_string();

    let checker = validate_password(&trimmed);

    if !checker.is_valid {
        println!("Password is not valid: ");
        if !checker.valid_min_length {
            println!("- Too short (min length: {})", 12);
        }

        if !checker.valid_lowercase {
            println!("- Must contain at least one lowercase letter");
        }

        let mut category_count = 0;
        if checker.valid_lowercase {
            category_count += 1;
        }
        if checker.valid_uppercase {
            category_count += 1;
        }
        if checker.valid_digits {
            category_count += 1;
        }
        if checker.valid_symbols {
            category_count += 1;
        }

        if category_count < 3 {
            println!("- Not enough character variety (need at least 3 types)")
        }

        return;
    }

    println!("Password is VALID ✅");
}
