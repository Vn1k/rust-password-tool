//Dictionary character (b = byte with type u8)
pub const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const DIGITS: &[u8] = b"0123456789";
pub const SYMBOLS: &[u8] = b"!@#$%^&*()_-+={}[];:,.?/";
pub const APP_NAME: &str = "rustpass";

pub const POOL_NUMERIC: f64 = 10.0;
pub const POOL_LOWER: f64 = 26.0;
pub const POOL_UPPER: f64 = 26.0;
pub const POOL_SYMBOL: f64 = 32.0;