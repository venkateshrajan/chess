use std::io::{stdout, Write};
use crate::board::*;

pub struct Move {
    pub from: usize,
    pub to: usize,
}

pub trait Strategy {
    fn next_move(&self, board: &Board) -> Move;
}

pub struct HumanStrategy {
    // user_interface: &'a dyn UI,
}

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
                    println!("{:?}. Please enter valid move.", e);
                    continue
                },
            };
        }
    }
}
