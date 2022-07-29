use crate::terminal::Terminal;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::{self};

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default(),
            cursor_position: Position::default(),
        }
    }

    fn die(&self, e: io::Error) {
        self.terminal.quit();
        panic!("{}", e);
    }

    pub fn run(&mut self) {
        self.draw_rows();
        loop {
            if self.should_quit {
                self.terminal.quit();
                break;
            }
            self.refresh_screen();
            match self.read_key() {
                Err(e) => self.die(e),
                Ok(pressed_key) => self.process_keypress(pressed_key),
            }
        }
    }

    fn read_key(&self) -> Result<KeyEvent, io::Error> {
        match read()? {
            Event::Key(pressed_key) => Ok(pressed_key),
            _ => panic!("not a key"),
        }
    }

    fn process_keypress(&mut self, pressed_key: KeyEvent) {
        match pressed_key.modifiers {
            KeyModifiers::CONTROL => match pressed_key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('s') => (),
                _ => (),
            },
            _ => match pressed_key.code {
                KeyCode::Char(c) => {
                    print!("{}", c);
                    self.move_cursor(KeyCode::Right);
                }
                KeyCode::Backspace => {
                    self.move_cursor(KeyCode::Left);
                    print!(" ");
                }
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right | KeyCode::Enter => {
                    self.move_cursor(pressed_key.code)
                }
                _ => (),
            },
        }
        Terminal::flush()
    }

    fn move_cursor(&mut self, keycode: KeyCode) {
        let Position { mut x, mut y } = self.cursor_position;
        match keycode {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = y.saturating_add(1),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = x.saturating_add(1),
            KeyCode::Enter => {
                x = 0;
                y = y.saturating_add(1)
            }
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }

    fn refresh_screen(&self) {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());
        // self.draw_rows();
        Terminal::cursor_position(&self.cursor_position);
        Terminal::cursor_show();
        Terminal::flush();
    }

    fn draw_rows(&self) {
        let height = self.terminal.size.height;
        for _ in 1..height {
            Terminal::clear_current_line();
            println!("~\r");
        }
    }
}
