use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use models::{TaskCommand, TaskCommandType, TaskStatus};

pub mod models;
mod task;

pub struct TaskManager {
    pub receiver: Receiver<TaskCommand>,
    sender: Sender<TaskCommand>,
    tasks: HashMap<String, task::Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        let (tx, rx) = channel::<TaskCommand>();
        TaskManager {
            tasks: HashMap::new(),
            sender: tx,
            receiver: rx,
        }
    }

    pub fn create_task(&mut self, name: String, timer: u16) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get timestamp, how is that even possible?")
            .as_secs()
            .to_string();

        let id = format!("{}_{}", name, timestamp);
        let new_task = task::Task::new(id.clone(), name, timer, self.sender.clone());
        self.tasks.insert(id.clone(), new_task);
        // self.sender
        //     .send(TaskCommand {
        //         id: id.clone(),
        //         status: TaskCommandType::StatusChanged(TaskStatus::Stopped),
        //     })
        //     .unwrap();

        id
    }

    pub fn start_task(&mut self, id: String, new_timer: Option<u16>) {
        let task = self.tasks.get_mut(&id).unwrap();
        // println!("start_task task: {}", id);
        task.start(new_timer);
    }

    pub fn pause_task(&mut self, id: String) {
        let task = self.tasks.get_mut(&id).unwrap();
        task.pause();
    }

    pub fn resume_task(&mut self, id: String) {
        let task = self.tasks.get_mut(&id).unwrap();
        task.resume();
    }

    pub fn stop_task(&mut self, id: String) {
        let task = self.tasks.get_mut(&id).unwrap();
        task.stop();
    }
}
