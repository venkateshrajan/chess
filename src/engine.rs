use crate::strategy::HumanStrategy;
use crate::board::*;
use crate::player::*;
use crate::ui::*;

pub trait Engine {
}

pub struct ConsoleEngine<'a> {
    p1: Player<'a>,
    p2: Player<'a>,
    board: Board,
    turn: bool,
    ui: &'a dyn UI,
}

impl<'a> ConsoleEngine<'a> {
    pub fn new() -> Self {
        Self {
            p1: Player { color: true, strategy: &HumanStrategy {} },
            p2: Player { color: false, strategy: &HumanStrategy {} },
            board: Board::new(),
            turn: true,
            ui: &ConsoleUI {},
        }
    }

    pub fn game_loop(&mut self) {
        while !self.is_game_over() {
            self.ui.display_board(&self.board);
            let curr_move = if self.turn {
                self.p1.next_move(&self.board)
            } else {
                self.p2.next_move(&self.board)
            };

            if let Err(e) = self.board.make_move(curr_move.from, curr_move.to) {
                panic!("Unable to move. Error {:?}", e);
            }

            self.turn = !self.turn;
        }
    }

    fn is_game_over(&self) -> bool {
        return false;
    }

    fn is_valid_move(&self) -> bool {
        return true;
    }
}
