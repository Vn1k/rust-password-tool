use crate::services::validate_password;

pub fn password_check(value: &str) {
    let data = value;

    if data.trim().is_empty() {
        println!("Program exit, no value given.");
        return;
    }

    let password = data.trim();
    let checker = validate_password(&password);

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
