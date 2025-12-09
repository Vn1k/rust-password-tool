use std::io;

pub fn confirmation_user(prompt: &str) -> bool {
    println!("{prompt} (y/n): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    matches!(input.trim().to_lowercase().as_str(), "y") //default true = "y"
}