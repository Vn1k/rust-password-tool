use rand::{Rng, rngs::OsRng};
use std::io;

//Dictionary character (b = byte with type u8)
const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn generate_password(length: usize) -> String {
    let mut rng = OsRng;
    let mut password = String::with_capacity(length);

    for _ in 0..length {
        let idx = rng.gen_range(0..CHARSET.len()); //get random from 0 to charset length
        let byte = CHARSET[idx]; //get the index of charset 
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

    let number: usize =  match input_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: input must be a number");
            return;
        },
    };

    if number < 4 {
        println!("Error: minimum length is 4");
        return;
    }

    let password = generate_password(number);

    println!("Length: {number}");
    println!("Generated Password: {password}");
}
