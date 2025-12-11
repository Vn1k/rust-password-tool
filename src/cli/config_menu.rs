use crate::services::{load_config, save_config};
use std::io::{self, Write};

pub fn config_menu() {
    let mut cfg = load_config();

    println!("Current config:");
    println!("Length    : {}", cfg.length);
    println!("Uppercase : {}", cfg.uppercase);
    println!("Digits    : {}", cfg.digits);
    println!("Symbols   : {}", cfg.symbols);
    println!();

    println!("Press ENTER to keep current value");

    fn read_line_trimmed() -> String {
        let mut input = String::new();
        io::stdout().flush().ok();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input_trim = input.trim().to_string();

        input_trim
    }

    //length
    println!("New length (current {}): ", cfg.length);
    let input = read_line_trimmed();
    if !input.is_empty() {
        if let Ok(len) = input.parse::<usize>() {
            if len >= 4 {
                cfg.length = len;
            } else {
                println!("Length too short, keeping previous value");
            }
        } else {
            println!("Invalid number, keeping previous value");
        }
    }

    //uppercase
    println!("Use uppercase? (y/n, current {}): ", cfg.uppercase);
    let input = read_line_trimmed();
    if !input.is_empty() {
        cfg.uppercase = matches!(input.to_lowercase().as_str(), "y");
    }

    // Digits
    print!("Use digits? (y/n, current {}): ", cfg.digits);
    let input = read_line_trimmed();
    if !input.is_empty() {
        cfg.digits = matches!(input.to_lowercase().as_str(), "y");
    }

    // Symbols
    print!("Use symbols? (y/n, current {}): ", cfg.symbols);
    let input = read_line_trimmed();
    if !input.is_empty() {
        cfg.symbols = matches!(input.to_lowercase().as_str(), "y");
    }

    if let Err(e) = save_config(&cfg) {
        eprintln!("Failed to save config: {e}");
    } else {
        println!("\nConfig updated and saved ✅");
    }
}
