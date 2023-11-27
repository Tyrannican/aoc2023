pub use clap::{Args, Parser};

#[derive(Parser, Debug)]
#[command(name = "Advent of Code")]
#[command(author = "Your name here <Edit src/cli.rs to add your name>")]
#[command(version = "1.0")]
#[command(about = "Run Advent of Code problems!")]
pub struct Cli {
    /// Problem to solve
    #[arg(short, long)]
    pub day: u8,
}
