pub mod config_menu;
pub mod password_generator;
pub mod password_validator;

use crate::models::{Cli, Commands};
use clap::Parser;
pub use config_menu::config_menu;
pub use password_generator::password_gen;
pub use password_validator::password_check;

pub fn main_cli() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Validate { value }) => {
            password_check(&value);
        }
        Some(Commands::Config) => {
            config_menu();
        }
        None => {
            password_gen();
        }
    }
}
