use std::fmt::Display;

use super::piece::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellColor {
    White,
    Black,
}

impl CellColor  {
    pub fn from_index(idx: usize) -> Option<CellColor> {
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
    pub color: CellColor,
    pub piece: Option<Piece>,
}

impl Default for BoardCell {
    fn default() -> Self {
        Self { color: CellColor::White, piece: None}
    }
}

impl Display for BoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Following link helps with ANSI Escape codes
        // https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

        // set background color
        if self.color == CellColor::White {
            write!(f, "\x1b[48;5;255m ")?;
        } else {
            write!(f, "\x1b[48;5;248m ")?;
        }

        if let Some(piece) = self.piece  {
            // set forground color
            write!(f, "\x1b[38;5;232m")?;
            write!(f, "{}", piece)?;
        } else {
            write!(f, " ")?;
        }

        // end color mode
        write!(f, " \x1b[0m")?;

        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternate_colors_on_row() {
        for i in 0..8 {
            assert_eq!(
                CellColor::from_index(i).unwrap(), 
                if i%2==0 {CellColor::White} else {CellColor::Black} 
            );
        }
    }

    #[test]
    fn alternate_colors_on_col() {
        for i in 0..8 {
            assert_eq!(
                CellColor::from_index(i*8).unwrap(), 
                if i%2==0 {CellColor::White} else {CellColor::Black} 
            );
        }
    }

    #[test]
    fn white_leading_diag() {
        for i in 0..8 {
            let cell_idx = i*8+i;
            assert_eq!(
                CellColor::from_index(cell_idx).unwrap(), 
                CellColor::White,
                "Value of i is {i}, cell index is {cell_idx}"
            );
        }
    }

    #[test]
    fn black_trailing_diag() {
        for i in 0..8 {
            let cell_idx = i*8+(8-i-1);
            assert_eq!(
                CellColor::from_index(cell_idx).unwrap(), 
                CellColor::Black,
                "Value of i is {i}, cell index is {cell_idx}"
            );
        }
    }
}
