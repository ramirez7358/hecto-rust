use std::io::{self, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.should_quit {
            println!("Goodbye.\r");
        }
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;

        if let Key::Ctrl('q') = pressed_key {
            self.should_quit = true;
        }
        Ok(())
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: &std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}
