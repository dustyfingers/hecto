use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn  to_ctrl_byte(c: char) -> u8 {
    let byte  = c as u8;
    // bitwise 'magic'
    byte & 0b0001_1111
}

fn die(e : std::io::Error) {
    panic!(e);
}

fn main() {
    // set terminal into 'raw' mode
    // instead of waiting for the enter key is pressed to process input
    let _stdout = stdout().into_raw_mode().unwrap();

    // read keypresses from user
    for b in io::stdin().bytes() {
        // b is the byte code of the character pressed!
        let b = b.unwrap();
        let c = b as char;

        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }

        // break from execution when crtl + q is pressed
        if b == to_ctrl_byte('q') {
            break;
        }
    }
}
