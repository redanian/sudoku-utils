use itertools::iproduct;

use crate::solving::traits::SudokuTemplateTransformer;
use crate::traits::SudokuTemplate;

pub(crate) struct SetNakedSingles;

impl SetNakedSingles {
    /// In each row or column, if a value can only exist in only one cell, sets it.
    fn in_rows_and_columns(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // For each cell
        for (x, y) in iproduct!(0..9, 0..9) {
            // If the cell is empty
            if sudoku.cells[x][y].is_empty() {
                // For each possible value of the cell
                for value in sudoku.cells[x][y].possible_values() {
                    let mut set_value_row = true;
                    let mut set_value_col = true;
                    // For each other cell in the same row or column
                    for o in 0..9 {
                        // If the other cell in the column is not the current cell
                        if x != o {
                            // If the other cell could also contain the value, then the value cannot be set
                            if sudoku.cells[o][y].possible_values().contains(&value) {
                                set_value_col = false;
                            }
                        }
                        // If the other cell in the row is not the current cell
                        if y != o {
                            // If the other cell could also contain the value, then the value cannot be set
                            if sudoku.cells[x][o].possible_values().contains(&value) {
                                set_value_row = false;
                            }
                        }

                        // If the value cannot be set neither because of the row, nor because of the column, break
                        if !set_value_row && !set_value_col {
                            break;
                        }
                    }
                    // If the value is only possible in the current cell
                    if set_value_row || set_value_col {
                        // Set it
                        sudoku.cells[x][y].set_value(value);
                        made_changes = true;
                        break;
                    }
                }
            }
        }

        made_changes
    }

    /// In each square, if a value can only exist in only one cell, sets it.
    fn in_squares(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // For each square
        for (sx, sy) in iproduct!([0, 3, 6], [0, 3, 6]) {
            // For each cell in the square
            for (x, y) in iproduct!(0..3, 0..3) {
                // If the cell is empty
                if sudoku.cells[sx + x][sy + y].is_empty() {
                    // For each possible value of the cell
                    for value in sudoku.cells[sx + x][sy + y].possible_values() {
                        let mut set_value = true;
                        // For each other cell in the same square
                        for (x2, y2) in iproduct!(0..3, 0..3) {
                            if !(x == x2 && y == y2) {
                                // If the other cell could also contain the value, then the value cannot be set
                                if sudoku.cells[sx + x2][sy + y2].possible_values().contains(&value) {
                                    set_value = false;
                                    break;
                                }
                            }
                        }
                        // If the value is only possible in the current cell
                        if set_value {
                            // Set it
                            sudoku.cells[sx + x][sy + y].set_value(value);
                            made_changes = true;
                            break;
                        }
                    }
                }
            }
        }

        made_changes
    }
}

impl SudokuTemplateTransformer for SetNakedSingles {
    fn transform(&self, sudoku: &mut SudokuTemplate) -> bool {
        SetNakedSingles::in_rows_and_columns(sudoku) ||
            SetNakedSingles::in_squares(sudoku)
    }
}
