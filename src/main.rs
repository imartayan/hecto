use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal,
};
use std::io::{stdout, Write};

fn main() {
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    println!("Hecto editor");

    loop {
        let event = read().expect("Failed to read event");
        if let Event::Key(pressed_key) = event {
            match pressed_key.modifiers {
                KeyModifiers::CONTROL => match pressed_key.code {
                    KeyCode::Char('q') => break,
                    _ => (),
                },
                _ => match pressed_key.code {
                    KeyCode::Char(c) => print!("{}", c),
                    _ => (),
                },
            }
        }

        stdout().flush().expect("Failed to flush stdout");
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
