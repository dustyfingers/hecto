
use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        // set terminal into 'raw' mode
        // instead of waiting for the enter key is pressed to process input
        let _stdout = stdout().into_raw_mode().unwrap();

        // read keypresses from user
        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    // this function constructs a new editor for us
    pub fn new() -> Self {
        Self{}
    }
    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program end"),
            _ => ()
        }
        Ok(())
    }

}