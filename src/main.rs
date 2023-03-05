pub mod board;

use board::*;

fn main() {
    let b = Board::new(); 
    println!("{}", b);
}
