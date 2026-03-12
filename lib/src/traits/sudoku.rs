use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use itertools::iproduct;
use crate::traits::SudokuTemplate;

/// Represents a Sudoku puzzle. Empty cells should be set as zero.
#[derive(Clone, Debug)]
pub struct Sudoku {
    pub(crate) cells: [[usize; 9]; 9],
}

impl Sudoku {
    pub fn empty() -> Self {
        Sudoku { cells: [[0; 9]; 9] }
    }

    /// Creates a new `Sudoku` instance from a 9x9 grid.
    pub fn new(cells: [[usize; 9]; 9]) -> Sudoku {
        Sudoku { cells }
    }

    pub fn get_cells(&self) -> &[[usize; 9]; 9] {
        &self.cells
    }

    pub fn contains_conflicts(&self) -> bool {
        // Check rows and columns.
        for i in 0..9 {
            let mut seen_in_row = [false; 10];
            let mut seen_in_col = [false; 10];
            for j in 0..9 {
                let row_value = self.cells[i][j];
                if row_value != 0 {
                    if seen_in_row[row_value] {
                        return true;
                    }
                    seen_in_row[row_value] = true;
                }

                let col_value = self.cells[j][i];
                if col_value != 0 {
                    if seen_in_col[col_value] {
                        return true;
                    }
                    seen_in_col[col_value] = true;
                }
            }
        }

        // Check squares.
        for (sx, sy) in iproduct!([0, 3, 6], [0, 3, 6]) {
            let mut seen = [false; 10];
            for (x, y) in iproduct!(0..3, 0..3) {
                let value = self.cells[sx + x][sy + y];
                if value != 0 {
                    if seen[value] {
                        return true;
                    }
                    seen[value] = true;
                }
            }
        }

        false
    }

    pub fn is_complete(&self) -> bool {
        self.cells
            .iter()
            .flatten()
            .all(|value| *value != 0)
    }

    pub fn is_solved(&self) -> bool {
        self.is_complete() && !self.contains_conflicts()
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

impl Sudoku {
    pub fn to_string(&self) -> String {
        self.cells
            .iter()
            .flatten()
            .map(|&n| {
                if n == 0 {
                    '.'
                } else {
                    char::from_digit(n as u32, 10).unwrap_or('.')
                }
            })
            .collect()
    }
}
