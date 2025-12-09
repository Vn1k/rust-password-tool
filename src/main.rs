use rand::{Rng, rngs::OsRng};

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
    let length = 10;
    let password = generate_password(length);
    println!("Generated Password: {password}");
}
