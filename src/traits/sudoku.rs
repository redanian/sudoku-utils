use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::traits::SudokuTemplate;

/// Represents a Sudoku puzzle. Empty cells should be set as zero.
#[derive(Clone, Debug)]
pub struct Sudoku {
    cells: [[usize; 9]; 9],
}

impl Sudoku {
    /// Creates a new `Sudoku` instance from a 9x9 grid.
    pub fn new(cells: [[usize; 9]; 9]) -> Sudoku {
        Sudoku { cells }
    }

    pub fn get_cells(&self) -> &[[usize; 9]; 9] {
        &self.cells
    }
}

impl From<SudokuTemplate> for Sudoku {
    fn from(sudoku_template: SudokuTemplate) -> Sudoku {
        let cells = sudoku_template.cells.map(|row| row.map(|cell| cell.get_value()));
        Sudoku::new(cells)
    }
}

#[derive(Debug)]
pub struct SudokuStrParsingError;

impl Display for SudokuStrParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Input is not 81 chars long")
    }
}

impl Error for SudokuStrParsingError {}

impl FromStr for Sudoku {
    type Err = SudokuStrParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 81 {
            return Err(SudokuStrParsingError);
        }

        let mut cells = [[0; 9]; 9];
        for (i1, (i2, c)) in s.chars().enumerate().enumerate() {
            let row = i1 / 9;
            let col = i2 % 9;

            cells[row][col] = c.to_digit(10).unwrap_or(0) as usize;
        }

        Ok(Sudoku::new(cells))
    }
}
