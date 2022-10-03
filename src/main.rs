use std::io::{self, Read};

fn main() {
    // read keypresses from user
    // heyooo
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        // break from execution when q is pressed
        if c == 'q' {
            break;
        }
    }
}
