use termion::raw::IntoRawMode;
use termion::scroll;
use termion::{color, style};

use std::io;

fn main() {
    println!("{}red", color::Fg(color::Red));
    println!("{}blue", color::Fg(color::Blue));
    println!("{}Yellow", color::Fg(color::Yellow));

    println!("{}Scroll", scroll::Up(12));

    println!("test");

    /* test */

    /* println!( */
    /* "{}{}Stuff", */
    /* termion::clear::All, */
    /* termion::cursor::Goto(1, 1) */
    /* ); */
}
