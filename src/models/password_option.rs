pub struct PasswordOption {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_digits: bool,
    pub use_symbols: bool,
}

impl PasswordOption {
    pub fn debug_print(&self) {
        println!("Length: {}", self.length);
        println!("Uppercase: {}", self.use_uppercase);
        println!("Digits: {}", self.use_digits);
        println!("Symbols: {}", self.use_symbols);
    }
}
