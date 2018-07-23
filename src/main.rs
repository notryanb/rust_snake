extern crate termion;

pub mod board;

use board::Board;
use termion::{color,style};

fn main() {
    let width = 50;
    let height = 30;
    let board = Board::new(width, height);

    println!("{}", board);


    // println!("{}{}", color::Fg(color::Red), test());
}
