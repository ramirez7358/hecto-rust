use std::io::{self, stdout, Write};
use termion::{
    color,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::editor::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        print!(
            "{}",
            termion::cursor::Goto(
                x.try_into().expect("Unexpected u16 overflow in Goto"),
                y.try_into().expect("Unexpected u16 overflow in Goto")
            )
        );
    }

    /// # Errors
    ///
    /// It is considered an error if not all bytes could be written due to I/O errors or EOF being reached.
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    /// # Errors
    ///
    /// It is considered an error if not all bytes could be written due to I/O errors or EOF being reached.
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
}

impl Default for Terminal {
    fn default() -> Self {
        let size = termion::terminal_size().unwrap();
        Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode().unwrap(),
        }
    }
}
