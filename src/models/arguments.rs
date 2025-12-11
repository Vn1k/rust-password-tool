use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>, // <---- OPTIONAL = boleh tanpa argumen
}

#[derive(Subcommand)]
pub enum Commands {
    Validate { value: String },
    Config,
}
