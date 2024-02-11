use crate::helpers::{loader::Loader, notifications};
use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use crate::task_manager::models::{TaskCommand, TaskCommandType, TaskStatus};

pub struct Task {
    pub id: String,
    pub name: String,
    pub timer: u16,
    status: Arc<Mutex<TaskStatus>>,
    internal_sender: Option<Sender<TaskStatus>>,
    external_sender: Sender<TaskCommand>,
}

impl Task {
    pub fn new(id: String, name: String, timer: u16, external_sender: Sender<TaskCommand>) -> Task {
        Task {
            id,
            name,
            timer,
            status: Arc::new(Mutex::new(TaskStatus::Stopped)),
            internal_sender: None,
            external_sender,
        }
    }

    pub fn start(&mut self, new_timer: Option<u16>) {
        let first_status_clone = Arc::clone(&self.status);

        let mut first_update_status = first_status_clone.lock().unwrap();
        if *first_update_status == TaskStatus::Running || (new_timer.is_none() && self.timer == 0) {
            return;
        }

        *first_update_status = TaskStatus::Running;

        if new_timer.is_some() {
            self.timer = new_timer.unwrap();
        }

        let (tx, rx) = channel::<TaskStatus>();
        let sender = self.external_sender.clone();
        let status_clone = Arc::clone(&self.status);

        self.internal_sender = Some(tx);
        let mut gnimit = self.timer;
        let current_id: String = self.id.clone();

        thread::spawn(move || loop {
            let mut status = status_clone.lock().unwrap();

            match rx.try_recv() {
                Ok(new_status) => match new_status {
                    TaskStatus::Paused => {
                        println!("Task paused");
                        *status = TaskStatus::Paused;
                    }
                    TaskStatus::Running => {
                        println!("Task resumed");
                        *status = TaskStatus::Running;
                    }
                    TaskStatus::Stopped => {
                        println!("Task stopped");
                        *status = TaskStatus::Stopped;
                        break;
                    }
                },
                _ => {}
            }

            if *status == TaskStatus::Running {
                gnimit -= 1;

                sender
                    .send(TaskCommand {
                        id: current_id.clone(),
                        status: TaskCommandType::Timeleft(gnimit),
                    })
                    .expect("Failed to send timeleft command, channel probably closed");

                if gnimit == 0 {
                    sender
                        .send(TaskCommand {
                            id: current_id.clone(),
                            status: TaskCommandType::StatusChanged(TaskStatus::Stopped),
                        })
                        .expect("Failed to send timeleft command, channel probably closed");
                    break;
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        self.external_sender
            .send(TaskCommand {
                id: self.id.clone(),
                status: TaskCommandType::StatusChanged(TaskStatus::Running),
            })
            .unwrap();
        println!("Task started-end");
    }

    pub fn pause(&mut self) {
        self.internal_sender
            .as_ref()
            .unwrap()
            .send(TaskStatus::Paused)
            .unwrap();
    }

    pub fn resume(&mut self) {
        self.internal_sender
            .as_ref()
            .unwrap()
            .send(TaskStatus::Running)
            .unwrap();
    }

    pub fn stop(&mut self) {
        self.internal_sender
            .as_ref()
            .unwrap()
            .send(TaskStatus::Stopped)
            .unwrap();
    }
}
