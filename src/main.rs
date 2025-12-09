use rand::{Rng, rngs::OsRng};
use std::io;

//Dictionary character (b = byte with type u8)
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()_-+={}[];:,.?/";

struct PasswordOption {
    length: usize,
    use_uppercase: bool,
    use_digits: bool,
    use_symbols: bool,
}

impl PasswordOption {
    fn debug_print(&self) {
        println!("Length: {}", self.length);
        println!("Uppercase: {}", self.use_uppercase);
        println!("Digits: {}", self.use_digits);
        println!("Symbols: {}", self.use_symbols);
    }
}

fn build_charset(options: &PasswordOption) -> Vec<u8> {
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

fn confirmation(prompt: &str) -> bool {
    println!("{prompt} (y/n): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    matches!(input.trim().to_lowercase().as_str(), "y")
}

fn generate_password(
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

fn main() {
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

    let use_uppercase = confirmation("Include uppercase letters?");
    let use_digits = confirmation("Include digtis?");
    let use_symbols = confirmation("Include symbols?");

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
