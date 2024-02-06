use crate::loader::Loader;
use std::process::Command;

pub fn open() {
    println!("Try");
    let mut loader = Loader::new();
    loader.start();

    let now = std::time::Instant::now();
    loop {
        if now.elapsed().as_secs() > 2 {
            loader.stop();
        }
        if now.elapsed().as_secs() > 10 {
            break;
        }
    }

    Command::new("osascript")
            .arg("-e")
            .arg("display notification \"Time has passed !!.\" with title \"Pomodoro Timer\" subtitle \"Ringing.\" sound name \"Frog\"")
            .spawn()
            .expect("Failed to open Terminal");
}
