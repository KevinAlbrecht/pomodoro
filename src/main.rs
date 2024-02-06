mod loader;
mod windows;

use clap::{command, Parser};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("Driiing") )]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    interval: u8,
}

fn main() {
    let args = Args::parse();

    windows::open();
}
