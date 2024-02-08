mod helpers;
mod task_manager;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("Driiing") )]
    name: String,

    #[arg(short, long, default_value_t = 1500)]
    time: u16,
}

fn main() {
    let args = Args::parse();
    let now = std::time::Instant::now();

    let mut task_manager = task_manager::TaskManager::new();

    task_manager.create_task(args.name, args.time);
    loop {}
}
