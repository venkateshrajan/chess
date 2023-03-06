mod cell;
mod piece;

use cell::*;
use piece::*;

#[derive(Debug, PartialEq)]
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
                    board.cells[idx].piece = match Piece::from_fen_char(c) {
                        Some(x) => Some(x),
                        None => return Err(format!("Invalid fen char {}", c)),
                    };
                    idx += 1;
                }
                '/' => {
                    if idx%8 != 0 { return Err(format!("Invalid fen string {}", fen)); }
                },
                _ => return Err(format!("Invalid fen char {}", c)),
            };
        }

        if idx != 64 { return Err(format!("Invalid fen string {}", fen)); }

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
    use pretty_assertions::assert_eq;

    #[test]
    fn new_board_creation() {
        let mut board = Board {
            cells: [Default::default(); 64],
        };

        for (i, cell) in board.cells.iter_mut().enumerate() {
            cell.color = CellColor::from_index(i).unwrap();
        }

        // Place Kings
        board.cells[4].piece = Piece::from_fen_char('k');
        board.cells[60].piece = Piece::from_fen_char('K');

        // Place Queens
        board.cells[3].piece = Piece::from_fen_char('q');
        board.cells[59].piece = Piece::from_fen_char('Q');

        // Place Rooks
        board.cells[0].piece = Piece::from_fen_char('r');
        board.cells[7].piece = Piece::from_fen_char('r');
        board.cells[56].piece = Piece::from_fen_char('R');
        board.cells[63].piece = Piece::from_fen_char('R');

        // Place Bishops
        board.cells[2].piece = Piece::from_fen_char('b');
        board.cells[5].piece = Piece::from_fen_char('b');
        board.cells[58].piece = Piece::from_fen_char('B');
        board.cells[61].piece = Piece::from_fen_char('B');

        // Place Knights
        board.cells[1].piece = Piece::from_fen_char('n');
        board.cells[6].piece = Piece::from_fen_char('n');
        board.cells[57].piece = Piece::from_fen_char('N');
        board.cells[62].piece = Piece::from_fen_char('N');
        
        // Place Pawns
        // Place black Pawns
        for i in 8..16 {
            board.cells[i].piece = Piece::from_fen_char('p');
        }
        // Place black Pawns
        for i in 48..56 {
            board.cells[i].piece = Piece::from_fen_char('P');
        }

        assert_eq!(board, Board::new());
    }

    #[test]
    fn creation_of_board_from_valid_fen_string() {
        // Thanks to http://bernd.bplaced.net/fengenerator/fengenerator.html
        let mut board = Board {
            cells: [Default::default(); 64],
        };

        for (i, cell) in board.cells.iter_mut().enumerate() {
            cell.color = CellColor::from_index(i).unwrap();
        }

        board.cells[3].piece = Piece::from_fen_char('Q');
        board.cells[6].piece = Piece::from_fen_char('B');
        board.cells[17].piece = Piece::from_fen_char('R');
        board.cells[23].piece = Piece::from_fen_char('P');
        board.cells[43].piece = Piece::from_fen_char('P');
        board.cells[44].piece = Piece::from_fen_char('k');
        board.cells[46].piece = Piece::from_fen_char('K');
        board.cells[48].piece = Piece::from_fen_char('r');

        assert_eq!(board, Board::from_fen("3Q2B1/8/1R5P/8/8/3Pk1K1/r7/8").unwrap());
    }

    #[test]
    #[should_panic(expected = "Invalid fen string")]
    fn creation_of_board_from_invalid_fen_string1() {
        Board::from_fen("3Q2B1/8/1R5P/8/8/3Pk1K1/r6/8").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid fen string")]
    fn creation_of_board_from_invalid_fen_string2() {
        Board::from_fen("3Q2B1/8/1R5P/8/8/3Pk1K1/r7/7").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid fen string")]
    fn creation_of_board_from_invalid_fen_string3() {
        Board::from_fen("3Q2B1/8/1R5/8/8/3Pk1K1/r7/8").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid fen char")]
    fn creation_of_board_from_invalid_fen_char1() {
        Board::from_fen("3Q2B1/8/1R5T/8/8/3Pk1K1/r7/8").unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid fen char")]
    fn creation_of_board_from_invalid_fen_char2() {
        Board::from_fen("3Q2B1/8/1R5P?8/8/3Pk1K1/r7/8").unwrap();
    }

    #[test]
    fn make_move_with_valid_indices() {
        let mut board = Board::from_fen("3Q2B1/8/1R5P/8/8/3Pk1K1/r7/8").unwrap();
        board.make_move(44, 36).unwrap();

        assert_eq!(board.cells[44].piece, None);
        assert_eq!(board.cells[36].piece, Piece::from_fen_char('k'));
    }

    #[test]
    #[should_panic(expected = "Invalid from index")]
    fn make_move_with_invalid_from_index() {
        let mut board = Board::from_fen("3Q2B1/8/1R5P/8/8/3Pk1K1/r7/8").unwrap();
        board.make_move(64, 36).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid to index")]
    fn make_move_with_invalid_to_index() {
        let mut board = Board::from_fen("3Q2B1/8/1R5P/8/8/3Pk1K1/r7/8").unwrap();
        board.make_move(44, 66).unwrap();
    }
}
