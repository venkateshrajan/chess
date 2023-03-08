use crate::strategy::*;
use crate::board::*;

pub struct Player<'a> {
    pub color: bool,
    pub strategy: &'a dyn Strategy,
}

impl<'a> Player<'a> {
    pub fn next_move(&self, board: &Board) -> Move {
        println!("Curent turn: {}", if self.color {"WHITE"} else {"BLACK"});
        self.strategy.next_move(board)
    }
}

