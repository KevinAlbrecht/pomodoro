mod helpers;
mod task_manager;
mod ui;

use clap::{command, Parser};
use std::{io::stdin, thread};
use crate::ui::printer::updateOut;
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
    let mut task_manager = task_manager::TaskManager::new();
    let mut tasks: Vec<String> = vec![];

    tasks.push(task_manager.create_task(args.name.clone(), args.time));
    task_manager.start_task(tasks[0].clone(), None);

    thread::spawn(move || loop {
        match task_manager.receiver.try_recv() {
            Ok(command) => match command.status {
                task_manager::models::TaskCommandType::Timeleft(timeleft) => {
                    // println!("Timer:\"{}\", Time left: {}", command.id, timeleft);
                    updateOut(vec![ui::printer::DisplayInformation {
                        name: command.id.clone(),
                        time: timeleft,
                        status: task_manager::models::TaskStatus::Running,
                    }]);
                }
                task_manager::models::TaskCommandType::StatusChanged(status) => {
                    // println!("Task status changed: {:?}", status.to_string());
                    updateOut(vec![ui::printer::DisplayInformation {
                        name: command.id.clone(),
                        time: 0,
                        status: task_manager::models::TaskStatus::Stopped,
                    }]);
                }
            },
            Err(_) => {} //do nothing, wait for more msgs to come
        }
    });

    loop {
        let mut entry = String::new();

        match stdin().read_line(&mut entry) {
            Ok(_) => {
                // match entry.trim() {
                //     "pause" => task_manager.pause_task(tasks[0].clone()),
                //     "resume" => task_manager.resume_task(tasks[0].clone()),
                //     "stop" => task_manager.stop_task(tasks[0].clone()),
                //     _ => println!("Unknown command"),
                // }
                // println!("react a {}", entry.trim());
                // break;
            }
            Err(e) => {
                println!("Error reading input: {}", e.to_string());
                break;
            }
        }
    }

    println!("Goodbye!")
}
