use std::{
    io::{stdout, Write},
    sync::mpsc::{channel, Sender},
    thread,
    time::Duration,
};

const ANIMATION_SLEEP_TIME: Duration = std::time::Duration::from_millis(70);

pub struct Loader {
    stop_sender: Option<Sender<bool>>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader { stop_sender: None }
    }

    pub fn start(&mut self) {
        let mut loader_status: u8 = 0;
        let (tx, rx) = channel::<bool>();
        self.stop_sender = Some(tx);

        thread::spawn(move || loop {
            let loader = match loader_status {
                0 => "|",
                1 => "/",
                2 => "-",
                3 => "\\",
                _ => "|",
            };
            write(format!("\r{}", loader));
            loader_status = (loader_status + 1) % 4;

            if rx.try_recv().is_ok() {
                write(format!("\r{}", " "));
                break;
            }
            std::thread::sleep(ANIMATION_SLEEP_TIME);
        });
    }

    pub fn stop(&mut self) {
        if self.stop_sender.is_none() {
            return;
        }

        self.stop_sender
            .take()
            .expect("Error while getting the channel")
            .send(true)
            .unwrap();
    }
}

fn write(str: String) {
    stdout().write(str.as_bytes()).unwrap();
    stdout().flush().unwrap();
}
