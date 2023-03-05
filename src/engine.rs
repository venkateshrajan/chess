use crate::board::*;

pub struct Player {
    color: bool,
}

pub struct Game {
    p1: Player,
    p2: Player,
    board: Board,
    turn: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            p1: Player { color: true },
            p2: Player { color: false },
            board: Board::new(),
            turn: true,
        }
    }

    pub fn game_loop(&self) {
        while !self.is_game_over() {
            println!("{}", self.board);
        }
    }

    fn is_game_over(&self) -> bool {
        return false;
    }
}
