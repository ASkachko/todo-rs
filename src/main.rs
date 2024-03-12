use termion::{color, style};

use std::io;

fn main() {
    println!("{}red", color::Fg(color::Red));
    println!("{}blue", color::Fg(color::Blue));
    println!("{}Yellow", color::Fg(color::Yellow));
}
