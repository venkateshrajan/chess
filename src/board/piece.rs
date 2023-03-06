use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: PieceColor,
}

impl Piece {
    pub fn from_fen_char(c: char) -> Option<Self> {
        match c {
            'K' => Some(Self { piece_type: PieceType::King, color: PieceColor::White }),
            'k' => Some(Self { piece_type: PieceType::King, color: PieceColor::Black }),
            'Q' => Some(Self { piece_type: PieceType::Queen, color: PieceColor::White }),
            'q' => Some(Self { piece_type: PieceType::Queen, color: PieceColor::Black }),
            'R' => Some(Self { piece_type: PieceType::Rook, color: PieceColor::White }),
            'r' => Some(Self { piece_type: PieceType::Rook, color: PieceColor::Black }),
            'B' => Some(Self { piece_type: PieceType::Bishop, color: PieceColor::White }),
            'b' => Some(Self { piece_type: PieceType::Bishop, color: PieceColor::Black }),
            'N' => Some(Self { piece_type: PieceType::Knight, color: PieceColor::White }),
            'n' => Some(Self { piece_type: PieceType::Knight, color: PieceColor::Black }),
            'P' => Some(Self { piece_type: PieceType::Pawn, color: PieceColor::White }),
            'p' => Some(Self { piece_type: PieceType::Pawn, color: PieceColor::Black }),
            _ => None,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // If you need more information about the following unicode characters
        // https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode
        write!(f, "{}", 
               match self.piece_type {
                   PieceType::King => if self.color == PieceColor::White { "\u{2654}" } else { "\u{265A}" },
                   PieceType::Queen => if self.color == PieceColor::White { "\u{2655}" } else { "\u{265B}" },
                   PieceType::Rook => if self.color == PieceColor::White { "\u{2656}" } else { "\u{265C}" },
                   PieceType::Bishop => if self.color == PieceColor::White { "\u{2657}" } else { "\u{265D}" },
                   PieceType::Knight => if self.color == PieceColor::White { "\u{2658}" } else { "\u{265E}" },
                   PieceType::Pawn => if self.color == PieceColor::White { "\u{2659}" } else { "\u{265F}" },
               }
        )?;

        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_fen_char() {
        let fen_chars = vec!['K', 'k', 'Q', 'q', 'R', 'r', 'B', 'b', 'N', 'n', 'P', 'p'];
        let res_pieces = vec![
            Piece { piece_type: PieceType::King, color: PieceColor::White },
            Piece { piece_type: PieceType::King, color: PieceColor::Black },
            Piece { piece_type: PieceType::Queen, color: PieceColor::White },
            Piece { piece_type: PieceType::Queen, color: PieceColor::Black },
            Piece { piece_type: PieceType::Rook, color: PieceColor::White },
            Piece { piece_type: PieceType::Rook, color: PieceColor::Black },
            Piece { piece_type: PieceType::Bishop, color: PieceColor::White },
            Piece { piece_type: PieceType::Bishop, color: PieceColor::Black },
            Piece { piece_type: PieceType::Knight, color: PieceColor::White },
            Piece { piece_type: PieceType::Knight, color: PieceColor::Black },
            Piece { piece_type: PieceType::Pawn, color: PieceColor::White },
            Piece { piece_type: PieceType::Pawn, color: PieceColor::Black },
        ];

        for i in 0..12 {
            assert_eq!(Piece::from_fen_char(fen_chars[i]).unwrap(), res_pieces[i]);
        }
    }
}
