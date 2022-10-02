use std::io::{self, Read};

fn main() {

    // read keypresses from user
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
    }
}
