use crate::helpers::{loader::Loader, notifications};
use std::{
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
    thread,
    time::Duration,
};

pub enum TaskStatus {
    Paused,
    Running,
    Stopped,
}
pub struct Task {
    pub id: String,
    pub name: String,
    pub timer: u16,
    pub status: TaskStatus,
    internal_sender: Option<Sender<TaskStatus>>,
}

impl Task {
    pub fn new(id: String, name: String, timer: u16) -> Task {
        Task {
            id,
            name,
            timer,
            status: TaskStatus::Stopped,
            internal_sender: None,
        }
    }

    pub fn start(&mut self, new_timer: Option<u16>) {
        if new_timer.is_none() && self.timer == 0 {
            return;
        } else if new_timer.is_some() {
            self.timer = new_timer.unwrap();
        }

        let now = std::time::Instant::now();
        let (tx, rx) = channel::<TaskStatus>();

        self.internal_sender = Some(tx);
        let mut gnimit = self.timer;

        thread::spawn(move || loop {
            match rx.try_recv() {
                Ok(status) => match status {
                    TaskStatus::Paused => {
                        println!("Task paused");
                    }
                    TaskStatus::Running => {
                        println!("Task resumed");
                    }
                    TaskStatus::Stopped => {
                        println!("Task stopped");
                        break;
                    }
                },
                _ => {
                    // if !self.is_paused && now.elapsed().as_millis() > 1000 {
                    gnimit -= 1;
                    println!("{} seconds left", gnimit);
                    if gnimit == 0 {
                        break;
                    }
                    //     thread::sleep(Duration::from_secs(1));
                    // }
                    thread::sleep(Duration::from_secs(1));
                }
            }
            // if !self.is_paused && now.elapsed().as_millis() > 1000 {
            //     self.timer -= 1;
            //     println!("{} seconds left", self.timer);
            //     thread::sleep(Duration::from_secs(1));
            // }
        });

        notifications::send_toast();
    }

    // A method that decrements the timer by 1
    // pub fn pause(&mut self) {
    //     self.is_paused = true;
    //     self.timer -= 1;
    // }

    // pub fn resume(&mut self) {
    //     self.is_paused = false;
    // }

    // pub fn open() {
    //     let now = std::time::Instant::now();
    //     let mut loader = Loader::new();

    //     loader.start();

    //     loop {
    //         if now.elapsed().as_millis() > 1000 {
    //             break;
    //         }
    //     }
    //     loader.stop();
    //     notifications::send_toast();
    // }
}
