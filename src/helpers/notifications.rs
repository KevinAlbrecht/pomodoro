use std::process::Command;

pub struct Notification {
    pub title: String,
    pub subtitle: Option<String>,
    pub message: String,
    pub sound: Option<String>,
}

pub fn send_toast() {
    let  res = Command::new("osascript")
    .arg("-e")
    .arg("display notification \"Time has passed !!.\" with title \"Pomodoro Timer\" subtitle \"Ringing.\" sound name \"Frog\"")
    .spawn()
    .expect("Failed to open Terminal");
}
