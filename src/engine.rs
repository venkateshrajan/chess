use std::io::{stdout, Write};

use crate::board::*;

struct Move {
    from: usize,
    to: usize,
}

trait Strategy {
    fn next_move(&self, board: &Board) -> Move;
}

struct HumanStrategy {}

impl HumanStrategy {
    fn parse_move(&self, user_input: &str) -> Result<Move, String> {
        const MOVE_LEN: usize = 4;
        let valid_pos = match user_input {
            _ if user_input.len() < MOVE_LEN => return Err(format!("Invalid input {user_input}")),
            validated => validated,
        };

        // get from indices
        let from_col = valid_pos.chars().nth(0).unwrap();
        let from_col_idx: usize = from_col as usize;
        let from_col_idx = match from_col_idx {
            _ if from_col_idx >= 'a' as usize && from_col_idx <= 'h' as usize =>
                from_col_idx - 'a' as usize,
            _ if from_col_idx >= 'A' as usize && from_col_idx <= 'H' as usize =>
                from_col_idx - 'A' as usize,
            _ => return Err(format!("Invalid from column index {from_col}")),
        };

        let from_row = valid_pos.chars().nth(1).unwrap();
        let from_row_idx: usize = from_row as usize;
        let from_row_idx = match from_row_idx {
            _ if from_row_idx >= '1' as usize && from_row_idx <= '8' as usize =>
                from_row_idx - '0' as usize,
            _ => return Err(format!("Invalid from row index {from_row}")),
        };
        

        // get to indices
        let to_col = valid_pos.chars().nth(2).unwrap();
        let to_col_idx: usize = to_col as usize;
        let to_col_idx = match to_col_idx {
            _ if to_col_idx >= 'a' as usize && to_col_idx <= 'h' as usize =>
                to_col_idx - 'a' as usize,
            _ if to_col_idx >= 'A' as usize && to_col_idx <= 'H' as usize =>
                to_col_idx - 'A' as usize,
            _ => return Err(format!("Invalid to column index {to_col}")),
        };

        let to_row = valid_pos.chars().nth(3).unwrap();
        let to_row_idx: usize = to_row as usize;
        let to_row_idx = match to_row_idx {
            _ if to_row_idx >= '1' as usize && to_row_idx <= '8' as usize =>
                to_row_idx - '0' as usize,
            _ => return Err(format!("Invalid to row index {to_row}")),
        };

        return Ok(Move {from: (8-from_row_idx)*8+from_col_idx, to: (8-to_row_idx)*8+to_col_idx});
    }
}

impl Strategy for HumanStrategy {
    fn next_move(&self, board: &Board) -> Move {
        println!("{}", board);

        let mut user_input = String::new();
        loop {
            print!("Type here:");
            if let Err(e) = stdout().flush() {
                panic!("Unable to flush the standard output. Error {:?}", e);
            }

            if let Err(e) = std::io::stdin().read_line(&mut user_input) {
                panic!("Unable to read input. Error {:?}", e);
            }

            match self.parse_move(&user_input.as_str()) {
                Ok(m) => return m,
                Err(e) => {
                    println!("Please enter valid move. Error: {:?}", e);
                    continue
                },
            };
        }
    }
}


struct Player<'a> {
    color: bool,
    strategy: &'a dyn Strategy,
}

impl<'a> Player<'a> {
    fn next_move(&self, board: &Board) -> Move {
        println!("Curent turn: {}", if self.color {"WHITE"} else {"BLACK"});
        self.strategy.next_move(board)
    }
}

pub struct Game<'a> {
    p1: Player<'a>,
    p2: Player<'a>,
    board: Board,
    turn: bool,
}

impl<'a> Game<'a> {
    pub fn new() -> Self {
        Self {
            p1: Player { color: true, strategy: &HumanStrategy {} },
            p2: Player { color: false, strategy: &HumanStrategy {} },
            board: Board::new(),
            turn: true,
        }
    }

    pub fn game_loop(&mut self) {
        while !self.is_game_over() {
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
}
