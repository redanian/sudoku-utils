use itertools::iproduct;

use crate::solving::traits::{Difficulty, SudokuSolvingStrategy};
use crate::traits::SudokuTemplate;

pub(crate) struct EliminatePossibilitiesUsingExistingSingles;

impl EliminatePossibilitiesUsingExistingSingles {
    /// For each cell that has a value, eliminates the value as a possibility from other cells in the same row or column.
    fn in_rows_and_columns(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // For each cell
        for (x, y) in iproduct!(0..9, 0..9) {
            // If the cell is set (has a value)
            if sudoku.cells[x][y].is_set() {
                let value = sudoku.cells[x][y].get_value();
                // For each other cell in the same row or column
                for o in 0..9 {
                    // If the other cell is not the current cell
                    if x != o {
                        // Remove the value of the current cell as a possibility
                        made_changes |= sudoku.cells[o][y].remove_possibility(value);
                    }
                    if y != o {
                        // Remove the value of the current cell as a possibility
                        made_changes |= sudoku.cells[x][o].remove_possibility(value);
                    }
                }
            }
        }

        made_changes
    }

    /// For each cell that has a value, eliminates the value as a possibility from other1 cells in the same square.
    fn in_squares(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // For each square
        for (sx, sy) in iproduct!([0, 3, 6], [0, 3, 6]) {
            // For each cell in the square
            for (x, y) in iproduct!(0..3, 0..3) {
                // If the cell is set (has a value)
                if sudoku.cells[sx + x][sy + y].is_set() {
                    let value = sudoku.cells[sx + x][sy + y].get_value();
                    // For each other cell in the same square
                    for (x2, y2) in iproduct!(0..3, 0..3) {
                        if !(x == x2 && y == y2) {
                            // Remove the value of the current cell as a possibility
                            made_changes |= sudoku.cells[sx + x2][sy + y2].remove_possibility(value);
                        }
                    }
                }
            }
        }

        made_changes
    }
}

impl SudokuSolvingStrategy for EliminatePossibilitiesUsingExistingSingles {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        EliminatePossibilitiesUsingExistingSingles::in_rows_and_columns(sudoku) ||
            EliminatePossibilitiesUsingExistingSingles::in_squares(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
}
