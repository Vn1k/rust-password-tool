use crate::config::load_config;
use crate::models::PasswordOption;
use crate::services::{build_charset, generate_password};

pub fn password_gen() {
    let cfg = load_config();
    let options: PasswordOption = cfg.password_options();
    let charset = build_charset(&options);
    let password = generate_password(options.length, &charset);

    println!("Generated Password: {password}");
}
