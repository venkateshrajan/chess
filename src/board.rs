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
struct BoardCell {
    color: CellColor,
    piece: Option<Piece>,
}

pub struct Board {
    cells: [BoardCell; 64]
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
           cells: [BoardCell { color: CellColor::White, piece: None}; 64]
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

        board
    }
}

use std::fmt;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            // print new line for each row
            if i%8 == 0 { writeln!(f, "")?; }

            // set color mode
            if cell.color == CellColor::White {
                write!(f, "\x1b[38;5;22m\x1b[48;5;194m ")?;
            } else {
                write!(f, "\x1b[38;5;194m\x1b[48;5;22m ")?;
            }

            if let Some(piece) = cell.piece  {
                write!(f, "{}", 
                       match piece.piece_type {
                           PieceType::King => if piece.color == PieceColor::White {"K"} else {"k"},
                           PieceType::Queen => if piece.color == PieceColor::White {"Q"} else {"q"},
                           PieceType::Rook => if piece.color == PieceColor::White {"R"} else {"r"},
                           PieceType::Bishop => if piece.color == PieceColor::White {"B"} else {"b"},
                           PieceType::Knight => if piece.color == PieceColor::White {"N"} else {"n"},
                           PieceType::Pawn => if piece.color == PieceColor::White {"P"} else {"p"},
                       }
                )?;
            } else {
                write!(f, " ")?;
            }

            // end color mode
            write!(f, " \x1b[0m")?;
        }
        fmt::Result::Ok(())
    }
}
