pub mod password_generator;
pub mod password_validator;

pub use password_generator::password_gen;
pub use password_validator::password_check;
use std::io;

pub fn main_ui() {
    println!("Please choose menu");
    println!("==================");
    println!("1. Password generator");
    println!("2. Password validator");
    println!("==================");

    let mut choose = String::new();

    io::stdin()
        .read_line(&mut choose)
        .expect("Failed to read input");

    match choose.trim() {
        "1" => {
            println!("Password generator");
            password_gen();
        }
        "2" => {
            println!("Password validator");
            password_check();
        }
        _ => {
            println!("Invalid number: {}", choose.trim());
        }
    }
}
