
use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


fn die(e: std::io::Error) {
    panic!("{}", e);
}

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        // set terminal into 'raw' mode
        // instead of waiting for the enter key is pressed to process input
        let _stdout = stdout().into_raw_mode().unwrap();

        // read keypresses from user
        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?} \r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }
                    },
                    // ctrl+q keybinding
                    Key::Ctrl('q') => break,
                    _ => println!("{:?}\r", key)
                },
                Err(err) => die(err)
            }
        }
    }
    // this function constructs a new editor for us
    pub fn default() -> Self {
        Self{}
    }
}