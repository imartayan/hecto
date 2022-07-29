use crate::editor::Position;
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::{stdout, Write};

#[derive(Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    pub size: Size,
}

impl Terminal {
    pub fn default() -> Self {
        terminal::enable_raw_mode().ok();
        Self::clear_screen();
        let size = terminal::size().unwrap();
        Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        }
    }

    pub fn quit(&self) {
        Self::cursor_position(&Position::default());
        Self::clear_screen();
        Self::clear_current_line();
        terminal::disable_raw_mode().ok();
    }

    pub fn flush() {
        stdout().flush().ok();
    }

    pub fn clear_screen() {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::Purge))
            .ok();
    }

    pub fn clear_current_line() {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::CurrentLine))
            .ok();
    }

    pub fn cursor_position(position: &Position) {
        stdout()
            .queue(cursor::MoveTo(position.x as u16, position.y as u16))
            .ok();
    }

    pub fn cursor_hide() {
        stdout().execute(cursor::DisableBlinking).ok();
    }

    pub fn cursor_show() {
        stdout().execute(cursor::EnableBlinking).ok();
    }
}
