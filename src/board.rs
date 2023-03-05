#[derive(Copy, Clone)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, PartialEq)]
enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    color: PieceColor,
}

#[derive(Copy, Clone, PartialEq)]
enum CellColor {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct BoardCell {
    color: CellColor,
    piece: Option<Piece>,
}

impl Default for BoardCell {
    fn default() -> Self {
        BoardCell { color: CellColor::White, piece: None}
    }
}

pub struct Board {
    cells: [BoardCell; 64]
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
           cells: [Default::default(); 64],
        };

        for (i, cell) in board.cells.iter_mut().enumerate() {
            if i%2 == 0 {
                if (i/8)%2==0 { 
                    cell.color = CellColor::White;
                } else {
                    cell.color = CellColor::Black;
                }
            } else { 
                if (i/8)%2==0 { 
                    cell.color = CellColor::Black;
                } else {
                    cell.color = CellColor::White;
                }
            }
        }
        // Place Kings
        board.cells[4].piece = Some(Piece {
            piece_type: PieceType::King,
            color: PieceColor::Black,
        });
        board.cells[60].piece = Some(Piece {
            piece_type: PieceType::King,
            color: PieceColor::White,
        });

        // Place Queens
        board.cells[3].piece = Some(Piece {
            piece_type: PieceType::Queen,
            color: PieceColor::Black,
        });
        board.cells[59].piece = Some(Piece {
            piece_type: PieceType::Queen,
            color: PieceColor::White,
        });

        // Place Rooks
        board.cells[0].piece = Some(Piece {
            piece_type: PieceType::Rook,
            color: PieceColor::Black,
        });
        board.cells[7].piece = Some(Piece {
            piece_type: PieceType::Rook,
            color: PieceColor::Black,
        });
        board.cells[56].piece = Some(Piece {
            piece_type: PieceType::Rook,
            color: PieceColor::White,
        });
        board.cells[63].piece = Some(Piece {
            piece_type: PieceType::Rook,
            color: PieceColor::White,
        });

        // Place Bishops
        board.cells[2].piece = Some(Piece {
            piece_type: PieceType::Bishop,
            color: PieceColor::Black,
        });
        board.cells[5].piece = Some(Piece {
            piece_type: PieceType::Bishop,
            color: PieceColor::Black,
        });
        board.cells[58].piece = Some(Piece {
            piece_type: PieceType::Bishop,
            color: PieceColor::White,
        });
        board.cells[61].piece = Some(Piece {
            piece_type: PieceType::Bishop,
            color: PieceColor::White,
        });

        // Place Knights
        board.cells[1].piece = Some(Piece {
            piece_type: PieceType::Knight,
            color: PieceColor::Black,
        });
        board.cells[6].piece = Some(Piece {
            piece_type: PieceType::Knight,
            color: PieceColor::Black,
        });
        board.cells[57].piece = Some(Piece {
            piece_type: PieceType::Knight,
            color: PieceColor::White,
        });
        board.cells[62].piece = Some(Piece {
            piece_type: PieceType::Knight,
            color: PieceColor::White,
        });
        
        // Place Pawns
        // Place black Pawns
        for i in 8..16 {
            board.cells[i].piece = Some(Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::Black,
            });
        }
        // Place black Pawns
        for i in 48..56 {
            board.cells[i].piece = Some(Piece {
                piece_type: PieceType::Pawn,
                color: PieceColor::White,
            });
        }

        board
    }

    pub fn _get(&self, pos: usize) -> Result<&BoardCell, String> {
        let valid_pos = match pos {
            _ if pos >= 64 => return Err(String::from("Invalid index {pos}")),
            validated => validated,
        };

        Ok(&self.cells[valid_pos])
    }

    pub fn _get_mut(&mut self, pos: usize) -> Result<&mut BoardCell, String> {
        let valid_pos = match pos {
            _ if pos >= 64 => return Err(String::from("Invalid index {pos}")),
            validated => validated,
        };

        Ok(&mut self.cells[valid_pos])
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

            // Following link helps with ANSI Escape codes
            // https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

            // set background color
            if cell.color == CellColor::White {
                write!(f, "\x1b[48;5;255m ")?;
            } else {
                write!(f, "\x1b[48;5;248m ")?;
            }


            if let Some(piece) = cell.piece  {
                // set forground color
                write!(f, "\x1b[38;5;232m")?;

                // If you need more information about the following unicode characters
                // https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode
                write!(f, "{}", 
                       match piece.piece_type {
                           PieceType::King => if piece.color == PieceColor::White { "\u{2654}" } else { "\u{265A}" },
                           PieceType::Queen => if piece.color == PieceColor::White { "\u{2655}" } else { "\u{265B}" },
                           PieceType::Rook => if piece.color == PieceColor::White { "\u{2656}" } else { "\u{265C}" },
                           PieceType::Bishop => if piece.color == PieceColor::White { "\u{2657}" } else { "\u{265D}" },
                           PieceType::Knight => if piece.color == PieceColor::White { "\u{2658}" } else { "\u{265E}" },
                           PieceType::Pawn => if piece.color == PieceColor::White { "\u{2659}" } else { "\u{265F}" },
                       }
                )?;
            } else {
                write!(f, " ")?;
            }

            // end color mode
            write!(f, " \x1b[0m")?;
        }
        writeln!(f, " 1")?; 

        fmt::Result::Ok(())
    }
}
