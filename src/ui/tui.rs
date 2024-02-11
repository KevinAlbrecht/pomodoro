use crate::task_manager::models::TaskStatus;
use ratatui::{
    layout::{Alignment, Rect},
    prelude::{CrosstermBackend, Terminal},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{
        block::{Position, Title},
        canvas::Label,
        Block, BorderType, Borders, Gauge, LineGauge, List, ListState, Padding, Paragraph, Wrap,
    },
    Frame,
};

use std::{
    io::{self, stderr, stdout, Write},
    os::unix::thread,
    sync::mpsc::{channel, Receiver, Sender},
};

pub struct DisplayInformation {
    pub name: String,
    pub time: u16,
    pub status: TaskStatus,
}

//     print!("\x1B[1J");

//     for info in information.iter() {
//         stdout()
//             .write(
//                 format!(
//                     "Timer:\"{}\":{}, Time left: {}\r",
//                     info.name,
//                     info.status.to_string(),
//                     info.time
//                 )
//                 .as_bytes(),
//             )
//             .unwrap();
//     }
//     io::stdout().flush().unwrap();
// }

pub enum DisplayType {
    Single,
}
pub struct Display {
    display_type: DisplayType,
    internal_sender: Sender<DisplayInformation>,
    internal_receiver: Receiver<DisplayInformation>,
}

impl Display {
    pub fn new(display_type: Option<DisplayType>) -> Display {
        let (tx, rx) = channel();
        let mut instance = Display {
            internal_sender: tx,
            internal_receiver: rx,
            display_type: display_type.unwrap_or(DisplayType::Single),
        };

        instance.init();

        instance
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
        let mut counter: i32 = 0;

        let handle = std::thread::spawn(move || {
            loop {
                terminal
                    .draw(|f| match self.display_type {
                        DisplayType::Single => draw_single_layout(
                            f,
                            0 as u8,
                            seconds_to_label(2549, false),
                            vec![
                                String::from("Step 1"),
                                String::from("Step 2"),
                                String::from("Step 3"),
                            ],
                            0,
                        ),
                    })
                    .unwrap();

                if crossterm::event::poll(std::time::Duration::from_millis(250)).unwrap() {
                    if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                            match key.code {
                                crossterm::event::KeyCode::Char('j') => continue,
                                crossterm::event::KeyCode::Char('k') => continue,
                                crossterm::event::KeyCode::Char('q') => break,
                                _ => {}
                            }
                        }
                    }
                }
            }

            crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)
                .unwrap();
            crossterm::terminal::disable_raw_mode().unwrap();
        });

        if let Err(e) = handle.join() {
            eprintln!("Error: {:?}", e);
        }
        Ok(())
    }

    // pub fn update(&self, information: Vec<DisplayInformation>) {
    //     for info in information.iter() {
    //         self.internal_sender.send(info.clone()).unwrap();
    //     }
    // }
}

// fn draw_layout(frame: &mut Frame) {
//     frame.render_widget(
//         Block::default()
//             .title(
//                 Title::from("Pomodoro")
//                     .position(Position::Top)
//                     .alignment(Alignment::Center),
//             )
//             .borders(Borders::ALL)
//             .border_style(Style::default().fg(Color::White))
//             .border_type(BorderType::Rounded)
//             .style(Style::default()),
//         Rect::new(0, 0, 30, 10),
//     )
// }

fn draw_single_layout(
    frame: &mut Frame,
    percentage: u8,
    label: String,
    steps: Vec<String>,
    current_step_index: usize,
) {
    let size = Rect::new(0, 0, 50, 10);
    let percentage_f = percentage as f64 / 100.0;
    // let mut list_state = ListState::default();
    // list_state.select(Some(current_step_index));

    frame.render_widget(
        Block::default()
            .title(
                Title::from("Pomodoro")
                    .position(Position::Top)
                    .alignment(Alignment::Center),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default()),
        Rect::from(size),
    );

    // frame.render_stateful_widget(
    //     List::new(steps.iter().map(|s| s.as_str()).collect::<Vec<&str>>())
    //         .block(Block::default().title("List").borders(Borders::ALL))
    //         .style(Style::default().fg(Color::White))
    //         .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
    //         .highlight_symbol(">>")
    //         .repeat_highlight_symbol(true)
    //         .direction(ratatui::widgets::ListDirection::TopToBottom),
    //     Rect::from(size),
    //     &mut list_state,
    // );

    frame.render_widget(
        Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .gauge_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .gauge_style(
                Style::default()
                    .fg(Color::Rgb(65, 156, 95))
                    .bg(Color::Rgb(217, 217, 217)),
            )
            .ratio(percentage_f)
            .label(label),
        Rect::new(0, size.height - 3, size.width, 3),
    )
}

fn seconds_to_label(seconds: u16, display_hours: bool) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let seconds = seconds % 60;

    let formatted_hours = if display_hours {
        format!("{:2}:", hours)
    } else {
        String::new()
    };
    format!("{}{:02}:{:02}", formatted_hours, minutes, seconds)
}
