use crate::solving::traits::SudokuSolvingStrategy;
use crate::traits::Difficulty;
use crate::traits::SudokuTemplate;
use itertools::{iproduct, Itertools};

pub(crate) struct EliminatePossibilitiesUsingPointing;

impl EliminatePossibilitiesUsingPointing {
    fn in_rows_and_columns(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        for row in 0..9 {
            let missing_values = sudoku.get_missing_values_in_row(row);
            for value in missing_values {
                let squares = (0..9)
                    .filter(|col| sudoku.cells[row][*col].possible_values().contains(&value))
                    .map(|col| col / 3)
                    .collect_vec();

                if !squares.is_empty() && squares.iter().all(|&col| col == squares[0]) {
                    let sq_row = row / 3;
                    let sq_col = squares[0];

                    for (x, y) in iproduct!(0..3, 0..3) {
                        let cell_row = 3 * sq_row + x;
                        let cell_col = 3 * sq_col + y;

                        if cell_row != row {
                            made_changes |= sudoku.cells[cell_row][cell_col].remove_possibility(value);
                        }
                    }
                }
            }
        }

        for col in 0..9 {
            let missing_values = sudoku.get_missing_values_in_column(col);
            for value in missing_values {
                let squares = (0..9)
                    .filter(|row| sudoku.cells[*row][col].possible_values().contains(&value))
                    .map(|row| row / 3)
                    .collect_vec();

                if !squares.is_empty() && squares.iter().all(|&row| row == squares[0]) {
                    let sq_row = squares[0];
                    let sq_col = col / 3;

                    for (x, y) in iproduct!(0..3, 0..3) {
                        let cell_row = 3 * sq_row + x;
                        let cell_col = 3 * sq_col + y;

                        if cell_col != col {
                            made_changes |= sudoku.cells[cell_row][cell_col].remove_possibility(value);
                        }
                    }
                }
            }
        }

        made_changes
    }

    /// In each square, if a missing value is only possible in a row or column subpart of the square, then it is not a
    /// possible value anymore for the parts of the row or column outside of the square.
    fn in_squares(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // For each square
        for (sq_row, sq_col) in iproduct!(0..3, 0..3) {
            let missing_values = sudoku.get_values_in_square(sq_row, sq_col);

            // For each missing value
            for value in missing_values {
                let mut value_in_one_row = true;
                let mut value_row = None;

                let mut value_in_one_col = true;
                let mut value_col = None;

                // For each cell in the square
                for (x, y) in iproduct!(0..3, 0..3) {
                    // Calculate row and column of the cell
                    let row = 3 * sq_row + x;
                    let col = 3 * sq_col + y;
                    let cell = &sudoku.cells[row][col];

                    // If cell is empty and `value` is a possibility
                    if cell.is_empty() && cell.possible_values().contains(&value) {
                        // Mark the first row and column where the value is a possibility
                        if value_row == None {
                            value_row = Some(row);
                        }
                        if value_col == None {
                            value_col = Some(col);
                        }

                        // Mark if the cells where the value is possible are part of a single row or column
                        value_in_one_row &= row == value_row.unwrap();
                        value_in_one_col &= col == value_col.unwrap();
                    }
                }

                // If the cells where the value is possible are part of a single row
                if value_in_one_row && value_row.is_some() {
                    let row = value_row.unwrap();
                    let columns_in_square = [3 * sq_col, 3 * sq_col + 1, 3 * sq_col + 2];
                    let columns_not_in_square = (0..9).filter(|c| !columns_in_square.contains(c)).collect_vec();

                    for col in columns_not_in_square {
                        made_changes |= sudoku.cells[row][col].remove_possibility(value);
                    }
                }

                // If the cells where the value is possible are part of a single column
                if value_in_one_col && value_col.is_some() {
                    let col = value_col.unwrap();
                    let rows_in_square = [3 * sq_row, 3 * sq_row + 1, 3 * sq_row + 2];
                    let rows_not_in_square = (0..9).filter(|c| !rows_in_square.contains(c)).collect_vec();

                    for row in rows_not_in_square {
                        made_changes |= sudoku.cells[row][col].remove_possibility(value);
                    }
                }
            }
        }

        made_changes
    }
}

impl SudokuSolvingStrategy for EliminatePossibilitiesUsingPointing {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        EliminatePossibilitiesUsingPointing::in_rows_and_columns(sudoku) ||
            EliminatePossibilitiesUsingPointing::in_squares(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }

    fn name(&self) -> &'static str {
        "EliminatePossibilitiesByDetectingPointingGroups"
    }
}
