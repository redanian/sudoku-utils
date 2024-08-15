use itertools::iproduct;

use crate::solving::traits::{Difficulty, SudokuSolvingStrategy};
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

impl SudokuSolvingStrategy for SetNakedSingles {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        SetNakedSingles::in_rows_and_columns(sudoku) || SetNakedSingles::in_squares(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
}

#[cfg(test)]
mod tests {
    use crate::solving::set_naked_singles::SetNakedSingles;
    use crate::solving::traits::{Difficulty, SudokuSolvingStrategy};
    use crate::traits::SudokuTemplate;
    use crate::Sudoku;
    use itertools::iproduct;

    const SUDOKU_WITH_NAKED_SINGLE_IN_ROW: &str = "\
        .23456789\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
    ";

    const SUDOKU_WITH_NAKED_SINGLE_IN_COLUMN: &str = "\
        .........\
        2........\
        3........\
        4........\
        5........\
        6........\
        7........\
        8........\
        9........\
    ";

    const SUDOKU_WITH_NAKED_SINGLE_IN_SQUARE: &str = "\
        .23......\
        456......\
        789......\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
    ";

    const SUDOKU_WITHOUT_NAKED_SINGLES: &str = "\
        123456789\
        ........1\
        ........2\
        ........3\
        ........4\
        ........5\
        ........6\
        ........7\
        ........8\
    ";

    #[test]
    fn in_rows_and_columns_correctly_sets_naked_single_in_rows() {
        // Given a sudoku with only one value missing in the first row.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITH_NAKED_SINGLE_IN_ROW.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for rows.
        let changed = SetNakedSingles::in_rows_and_columns(&mut sudoku);

        // Then the missing value in the first row should be filled, and all other cells should remain unchanged.
        assert_eq!(changed, true);
        assert_eq!(sudoku.cells[0][0].get_value(), 1);

        for (x, y) in iproduct!(0..9, 0..9) {
            if (x, y) == (0, 0) {
                continue
            }
            assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.");
        }
    }

    #[test]
    fn in_rows_and_columns_correctly_sets_naked_single_in_columns() {
        // Given a sudoku with only one value missing in the first column.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITH_NAKED_SINGLE_IN_COLUMN.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for columns.
        let changed = SetNakedSingles::in_rows_and_columns(&mut sudoku);

        // Then the missing value in the first column should be filled, and all other cells should remain unchanged.
        assert_eq!(changed, true);
        assert_eq!(sudoku.cells[0][0].get_value(), 1);

        for (x, y) in iproduct!(0..9, 0..9) {
            if (x, y) == (0, 0) {
                continue
            }
            assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.");
        }
    }

    #[test]
    fn in_rows_and_columns_does_not_modify_sudoku_without_naked_singles() {
        // Given a sudoku without naked singles.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITHOUT_NAKED_SINGLES.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for rows and columns.
        let changed = SetNakedSingles::in_rows_and_columns(&mut sudoku);

        // Then the sudoku should remain unchanged.
        assert_eq!(changed, false);
        assert_eq!(sudoku, original);
    }

    #[test]
    fn in_squares_correctly_sets_naked_single_in_squares() {
        // Given a sudoku with only one value missing in the first square.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITH_NAKED_SINGLE_IN_SQUARE.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for squares.
        let changed = SetNakedSingles::in_squares(&mut sudoku);

        // Then the missing value in the first square should be filled, and all other cells should remain unchanged.
        assert_eq!(changed, true);
        assert_eq!(sudoku.cells[0][0].get_value(), 1);

        for (x, y) in iproduct!(0..9, 0..9) {
            if (x, y) == (0, 0) {
                continue
            }
            assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.");
        }
    }

    #[test]
    fn in_squares_does_not_modify_sudoku_without_naked_singles() {
        // Given a sudoku without naked singles.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITHOUT_NAKED_SINGLES.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for squares.
        let changed = SetNakedSingles::in_squares(&mut sudoku);

        // Then the sudoku should remain unchanged.
        assert_eq!(changed, false);
        assert_eq!(sudoku, original);
    }

    #[test]
    fn solve_correctly_returns_changed_flag() {
        // Given sudokus with naked singles.
        let mut sudoku1 = SudokuTemplate::from(SUDOKU_WITH_NAKED_SINGLE_IN_ROW.parse::<Sudoku>().unwrap());
        let mut sudoku2 = SudokuTemplate::from(SUDOKU_WITH_NAKED_SINGLE_IN_COLUMN.parse::<Sudoku>().unwrap());
        let mut sudoku3 = SudokuTemplate::from(SUDOKU_WITH_NAKED_SINGLE_IN_SQUARE.parse::<Sudoku>().unwrap());

        // When I apply the strategy using solve(), then it should return true.
        assert_eq!(SetNakedSingles {}.solve(&mut sudoku1), true);
        assert_eq!(SetNakedSingles {}.solve(&mut sudoku2), true);
        assert_eq!(SetNakedSingles {}.solve(&mut sudoku3), true);

        // Given a sudoku without naked singles.
        let mut sudoku4 = SudokuTemplate::from(SUDOKU_WITHOUT_NAKED_SINGLES.parse::<Sudoku>().unwrap());

        // When I apply the strategy using solve(), then it should return false.
        assert_eq!(SetNakedSingles {}.solve(&mut sudoku4), false);
    }

    #[test]
    fn difficulty_is_easy() {
        assert_eq!(SetNakedSingles {}.difficulty(), Difficulty::Easy);
    }
}
