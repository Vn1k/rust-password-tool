use std::io;

use crate::services::validate_password;

fn scoring(score: f64) {
    if score >= 75.0 {
        println!("Category password: Excellent");
        println!("Password strength: {:.2}", score)
    } else if score >= 50.0 {
        println!("Category password: Good");
        println!("Password strength: {:.2}", score)
    } else if score >= 25.0 {
        println!("Category password: Weak");
        println!("Password strength: {:.2}", score)
    } else {
        println!("Category password: Poor");
        println!("Password strength: {:.2}", score)
    }
}

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
    scoring(checker.entropy_score);

    if !checker.is_valid {
        println!("Password is not valid: ");
        if !checker.valid_min_length {
            println!("- Too short (min length: {})", 12);
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
            println!("- Not enough character variety (need at least 3 types).");
            println!("  Missing types:");

            if !checker.valid_lowercase { println!("   * Lowercase letters (a-z)")}
            if !checker.valid_uppercase { println!("   * Uppercase letters (A-Z)"); }
            if !checker.valid_digits { println!("   * Numbers (0-9)"); }
            if !checker.valid_symbols { println!("   * Symbols (!@#$...)"); }
        }

        return;
    }

    println!("Password is VALID ✅");
}
