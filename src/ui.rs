use crate::board::*;
use crate::board::cell::*;
use crate::board::piece::*;

pub trait UI {
    fn display_board(&self, board: &Board);
    fn display_cell(&self, cell: &BoardCell);
    fn display_piece(&self, piece: &Piece);
    fn show_winner(&self, winner: bool);
}

struct ConsoleUI {}

impl UI for ConsoleUI {
    fn display_board(&self, board: &Board) {
        for c in 65..73 {
            println!(" {} ", char::from_u32(c).unwrap());
        }

        for (i, cell) in board.cells.iter().enumerate() {
            // print new line for each row
            if i%8 == 0 { 
                if i < 8 {
                    println!(""); 
                } else {
                    println!(" {}", 8-(i/8)+1); 
                }
            }

            self.display_cell(cell);
        }
        println!(" 1"); 
    }

    fn show_winner(&self, winner: bool) {
    }

    fn display_cell(&self, cell: &BoardCell) {
        // Following link helps with ANSI Escape codes
        // https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

        // set background color
        if cell.color == CellColor::White {
            print!("\x1b[48;5;255m ");
        } else {
            print!("\x1b[48;5;248m ");
        }

        if let Some(piece) = cell.piece  {
            // set forground color
            print!("\x1b[38;5;232m");
            self.display_piece(&piece);
        } else {
            print!(" ");
        }

        // end color mode
        print!(" \x1b[0m");
    }

    fn display_piece(&self, piece: &Piece) {
        // If you need more information about the following unicode characters
        // https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode
        print!("{}", 
               match piece.piece_type {
                   PieceType::King => if piece.color == PieceColor::White { "\u{2654}" } else { "\u{265A}" },
                   PieceType::Queen => if piece.color == PieceColor::White { "\u{2655}" } else { "\u{265B}" },
                   PieceType::Rook => if piece.color == PieceColor::White { "\u{2656}" } else { "\u{265C}" },
                   PieceType::Bishop => if piece.color == PieceColor::White { "\u{2657}" } else { "\u{265D}" },
                   PieceType::Knight => if piece.color == PieceColor::White { "\u{2658}" } else { "\u{265E}" },
                   PieceType::Pawn => if piece.color == PieceColor::White { "\u{2659}" } else { "\u{265F}" },
               }
        );
    }
}
