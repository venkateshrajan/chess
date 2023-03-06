mod cell;
mod piece;

use cell::*;
use piece::*;

pub struct Board {
    cells: [BoardCell; 64]
}

impl Board {
    pub fn new() -> Self {
        if let Ok(board) = Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR") {
            return board;
        }

        Self {
            cells: [Default::default(); 64],
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut board = Self {
           cells: [Default::default(); 64],
        };

        let mut idx: usize = 0;
        for c in fen.chars() {
            match c {
                _ if c.is_numeric() => {
                    for i in idx..idx+(c.to_digit(10).unwrap() as usize) {
                        board.cells[i].color = CellColor::from_index(i).unwrap();
                        idx += 1;
                    }
                }, 
                _ if c.is_alphabetic() => {
                    board.cells[idx].color = CellColor::from_index(idx).unwrap();
                    board.cells[idx].piece = Piece::from_fen_char(c);
                    idx += 1;
                }
                '/' => {
                    if idx%8 != 0 { return Err(format!("Invalid fen string {}", fen)); }
                },
                _ => return Err(format!("Invalid fern char: {c}")),
            };
        }

        Ok(board)
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> Result<(), String> {
        let valid_from = match from {
            _ if from >= 64 => return Err(String::from("Invalid from index {from}")),
            validated => validated,
        };
        let valid_to = match to {
            _ if to >= 64 => return Err(String::from("Invalid to index {to}")),
            validated => validated,
        };

        self.cells[valid_to].piece = self.cells[valid_from].piece;
        self.cells[valid_from].piece = None;

        Ok(())
    }
}

use std::fmt;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in 65..73 {
            write!(f, " {} ", char::from_u32(c).unwrap())?;
        }

        for (i, cell) in self.cells.iter().enumerate() {
            // print new line for each row
            if i%8 == 0 { 
                if i < 8 {
                    writeln!(f, "")?; 
                } else {
                    writeln!(f, " {}", 8-(i/8)+1)?; 
                }
            }
            write!(f, "{}", cell)?;
        }
        writeln!(f, " 1")?; 

        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_board_creation() {
        unimplemented!();
    }

    #[test]
    fn creation_of_board_from_valid_fen_string() {
        unimplemented!();
    }

    #[test]
    fn creation_of_board_from_invalid_fen_string() {
        unimplemented!();
    }

    #[test]
    fn creation_of_board_from_invalid_fen_char() {
        unimplemented!();
    }

    #[test]
    fn make_move() {
        unimplemented!();
    }
}
