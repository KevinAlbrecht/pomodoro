pub enum TaskCommandType {
    Timeleft(u16),
    StatusChanged(TaskStatus),
}

pub struct TaskCommand {
    pub id: String,
    pub status: TaskCommandType,
}

#[derive(PartialEq)]
pub enum TaskStatus {
    Paused,
    Running,
    Stopped,
}
impl TaskStatus {
    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::Paused => String::from("Paused"),
            TaskStatus::Running => String::from("Running"),
            TaskStatus::Stopped => String::from("Stopped"),
        }
    }
}
