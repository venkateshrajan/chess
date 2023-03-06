mod piece;

use piece::*;

#[derive(Copy, Clone, PartialEq)]
enum CellColor {
    White,
    Black,
}

impl CellColor  {
    fn from_index(idx: usize) -> Option<CellColor> {
        if idx > 63 { return None; }

        if idx%2 == 0 {
            if (idx/8)%2==0 { 
                return Some(CellColor::White);
            } else {
                return Some(CellColor::Black);
            }
        } else { 
            if (idx/8)%2==0 { 
                return Some(CellColor::Black);
            } else {
                return Some(CellColor::White);
            }
        }
    }
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
                write!(f, "{}", piece)?;
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
