use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustpass")]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Validate,
    Config,
}
