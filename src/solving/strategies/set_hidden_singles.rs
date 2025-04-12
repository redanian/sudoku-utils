use crate::solving::traits::SudokuSolvingStrategy;
use crate::traits::Difficulty;
use crate::traits::SudokuTemplate;
use itertools::iproduct;

/// Sudoku strategy that sets hidden singles in a sudoku puzzle by analyzing possible values of empty cells. It compares
/// the possibilities of related empty cells, and if a value exists as a possibility only in one empty cell, it sets
/// that value to that cell.
///
/// Consider the following example:
/// ```text
///    1 2 3 4 5 6 7 8 9
///    -----------------
/// A |     |    5|     |
/// B |     |     |     |
/// C |3 4 1|2    |  6 8|
///    -----------------
/// D |     |     |     |
/// E |     |     |     |
/// F |     |     |     |
///    -----------------
/// G |     |     |     |
/// H |     |     |     |
/// I |     |     |     |
///    -----------------
/// ```
/// The row `C` is missing the values [5, 7, 9], so the cells `C5`, `C6` and `C7` have [5, 7, 9] as possible values.
/// However, because of the number 5 at `A6`, 5 is not an option for `C5` and `C6` due to these cells being in the same
/// block. This means that although `C7` has possible values [5, 7, 9], it is the only cell in row `C` that can contain
/// the number 5. This is called a hidden single.
///
/// This strategy implementation iterates through all possible values of all empty cells in the sudoku. For each
/// possible value, it checks weather the value is also possible in other related empty cells. If not, it then sets the
/// value to the only empty cell that can contain it.
pub(crate) struct SetHiddenSingles;

impl SetHiddenSingles {
    /// For each possible value of each empty cell, it sets the value to the cell if the value is only possible in the
    /// cell and not in other empty cells in the same row or column.
    fn in_rows_and_columns(sudoku: &mut SudokuTemplate) -> bool {
        // For each cell
        for (x, y) in iproduct!(0..9, 0..9) {
            // If the cell is empty
            if sudoku.cells[x][y].is_empty() {
                // For each possible value of the cell
                for value in sudoku.cells[x][y].possible_values() {
                    // Suppose the value can only be set in the current cell
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
                        return true;
                    }
                }
            }
        }

        false
    }

    /// For each square, for each possible value of each empty cell in the square, it sets the value to the cell if
    /// the value is only possible in the cell and not in other empty cells in the same square.
    fn in_squares(sudoku: &mut SudokuTemplate) -> bool {
        // For each square
        for (sx, sy) in iproduct!([0, 3, 6], [0, 3, 6]) {
            // For each cell in the square
            for (x, y) in iproduct!(0..3, 0..3) {
                // If the cell is empty
                if sudoku.cells[sx + x][sy + y].is_empty() {
                    // For each possible value of the cell
                    for value in sudoku.cells[sx + x][sy + y].possible_values() {
                        // Suppose the value can only be set in the current cell
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
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

impl SudokuSolvingStrategy for SetHiddenSingles {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        SetHiddenSingles::in_rows_and_columns(sudoku) || SetHiddenSingles::in_squares(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
}

#[cfg(test)]
mod tests {
    use crate::solving::strategies::set_hidden_singles::SetHiddenSingles;
    use crate::solving::traits::SudokuSolvingStrategy;
    use crate::traits::SudokuTemplate;
    use crate::Difficulty;
    use crate::Sudoku;
    use itertools::iproduct;

    const SUDOKU_WITH_HIDDEN_SINGLE_IN_ROW: &str = "\
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

    const SUDOKU_WITH_HIDDEN_SINGLE_IN_COLUMN: &str = "\
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

    const SUDOKU_WITH_HIDDEN_SINGLE_IN_SQUARE: &str = "\
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

    const SUDOKU_WITHOUT_HIDDEN_SINGLES: &str = "\
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
    fn in_rows_and_columns_correctly_sets_hidden_single_in_rows() {
        // Given a sudoku with only one value missing in the first row.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITH_HIDDEN_SINGLE_IN_ROW.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for rows.
        let changed = SetHiddenSingles::in_rows_and_columns(&mut sudoku);

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
    fn in_rows_and_columns_correctly_sets_hidden_single_in_columns() {
        // Given a sudoku with only one value missing in the first column.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITH_HIDDEN_SINGLE_IN_COLUMN.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for columns.
        let changed = SetHiddenSingles::in_rows_and_columns(&mut sudoku);

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
    fn in_rows_and_columns_does_not_modify_sudoku_without_hidden_singles() {
        // Given a sudoku without hidden singles.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITHOUT_HIDDEN_SINGLES.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for rows and columns.
        let changed = SetHiddenSingles::in_rows_and_columns(&mut sudoku);

        // Then the sudoku should remain unchanged.
        assert_eq!(changed, false);
        assert_eq!(sudoku, original);
    }

    #[test]
    fn in_squares_correctly_sets_hidden_single_in_squares() {
        // Given a sudoku with only one value missing in the first square.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITH_HIDDEN_SINGLE_IN_SQUARE.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for squares.
        let changed = SetHiddenSingles::in_squares(&mut sudoku);

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
    fn in_squares_does_not_modify_sudoku_without_hidden_singles() {
        // Given a sudoku without hidden singles.
        let mut sudoku = SudokuTemplate::from(SUDOKU_WITHOUT_HIDDEN_SINGLES.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for squares.
        let changed = SetHiddenSingles::in_squares(&mut sudoku);

        // Then the sudoku should remain unchanged.
        assert_eq!(changed, false);
        assert_eq!(sudoku, original);
    }

    #[test]
    fn solve_correctly_returns_changed_flag() {
        // Given sudokus with hidden singles.
        let mut sudoku1 = SudokuTemplate::from(SUDOKU_WITH_HIDDEN_SINGLE_IN_ROW.parse::<Sudoku>().unwrap());
        let mut sudoku2 = SudokuTemplate::from(SUDOKU_WITH_HIDDEN_SINGLE_IN_COLUMN.parse::<Sudoku>().unwrap());
        let mut sudoku3 = SudokuTemplate::from(SUDOKU_WITH_HIDDEN_SINGLE_IN_SQUARE.parse::<Sudoku>().unwrap());

        // When I apply the strategy using solve(), then it should return true.
        assert_eq!(SetHiddenSingles {}.solve(&mut sudoku1), true);
        assert_eq!(SetHiddenSingles {}.solve(&mut sudoku2), true);
        assert_eq!(SetHiddenSingles {}.solve(&mut sudoku3), true);

        // Given a sudoku without hidden singles.
        let mut sudoku4 = SudokuTemplate::from(SUDOKU_WITHOUT_HIDDEN_SINGLES.parse::<Sudoku>().unwrap());

        // When I apply the strategy using solve(), then it should return false.
        assert_eq!(SetHiddenSingles {}.solve(&mut sudoku4), false);
    }

    #[test]
    fn difficulty_is_easy() {
        assert_eq!(SetHiddenSingles {}.difficulty(), Difficulty::Easy);
    }
}
