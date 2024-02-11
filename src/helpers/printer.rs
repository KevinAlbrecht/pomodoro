use crate::task_manager::models::TaskStatus;
use std::io::{self, stdout, Write};

pub struct DisplayInformation {
    pub name: String,
    pub time: u16,
    pub status: TaskStatus,
}
pub fn updateOut(information: Vec<DisplayInformation>) {
    print!("\x1B[1J");

    for info in information.iter() {
        stdout()
            .write(
                format!(
                    "Timer:\"{}\":{}, Time left: {}\r",
                    info.name,
                    info.status.to_string(),
                    info.time
                )
                .as_bytes(),
            )
            .unwrap();
    }
    io::stdout().flush().unwrap();
}
