use crate::solving::traits::SudokuSolvingStrategy;
use crate::traits::Difficulty;
use crate::traits::SudokuTemplate;
use itertools::iproduct;

/// Sudoku strategy that eliminates possibilities in a sudoku puzzle by analyzing existing single values. It uses cells
/// that already have a value to remove that value as a possibility from other cells in the same row, column, or block.
///
/// Consider the following example:
/// ```text
///    1 2 3 4 5 6 7 8 9
///    -----------------
/// A |5    |     |     |
/// B |     |     |     |
/// C |     |     |     |
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
/// The cell at `A1` has the value 5, so other cells in the row `A`, column `1` or block `A-C, 1-3` cannot have the
/// value 5 and this value can safely be eliminated as a possibility from these cells.
///
/// This strategy implementation iterates through all cells in the sudoku. When a cell with a confirmed value is found,
/// it updates the candidates of all related cells (same row, column, or block) by removing the confirmed value.
pub(crate) struct EliminatePossibilitiesUsingExistingSingles;

impl EliminatePossibilitiesUsingExistingSingles {
    /// For each cell that has a value, eliminates the value as a possibility from other cells in the same row or
    /// column.
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

    /// For each cell that has a value, eliminates the value as a possibility from other cells in the same square.
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

#[cfg(test)]
mod tests {
    use crate::solving::strategies::eliminate_possibilities_using_existing_singles::EliminatePossibilitiesUsingExistingSingles;
    use crate::solving::traits::SudokuSolvingStrategy;
    use crate::traits::Difficulty;
    use crate::traits::SudokuTemplate;
    use crate::Sudoku;
    use itertools::iproduct;

    const EMPTY_SUDOKU: &str = "\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
    ";

    const SUDOKU: &str = "\
        1........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
    ";

    #[test]
    fn in_rows_and_columns_correctly_removes_possibilities_for_existing_value() {
        // Given a sudoku with only one cell with a value in the first row and column.
        let mut sudoku = SudokuTemplate::from(SUDOKU.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for rows and columns.
        let changed = EliminatePossibilitiesUsingExistingSingles::in_rows_and_columns(&mut sudoku);

        // Then the cell should not be changed, the value should be removed as a possibility from other cells in the
        // same row and column and all other cells should remain unchanged.
        assert_eq!(changed, true, "Sudoku template should have changed but was not.");
        assert_eq!(sudoku.cells[0][0].get_value(), 1, "The value at cell (0, 0) should be 1 but is not.");

        for (x, y) in iproduct!(0..9, 0..9) {
            if (x, y) == (0, 0) {
                continue
            } else if x == 0 || y == 0 {
                // For related cells:
                assert_eq!(
                    sudoku.cells[x][y].contains_possibility(1),
                    false,
                    "Cell at ({x}, {y}) still contains the value 1 as a possibility."
                );
            } else {
                // For unrelated cells:
                assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.");
            }
        }
    }

    #[test]
    fn in_rows_and_columns_does_not_change_empty_sudoku() {
        // Given an empty sudoku.
        let mut sudoku = SudokuTemplate::from(EMPTY_SUDOKU.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for rows and columns.
        let changed = EliminatePossibilitiesUsingExistingSingles::in_rows_and_columns(&mut sudoku);

        // Then the sudoku should not have changed.
        assert_eq!(changed, false, "Sudoku template should not have changed.");
        assert_eq!(sudoku, original, "Sudoku template should not have changed.");
    }

    #[test]
    fn in_squares_correctly_removes_possibilities_for_existing_value() {
        // Given a sudoku with only one cell with a value in the first square.
        let mut sudoku = SudokuTemplate::from(SUDOKU.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for squares.
        let changed = EliminatePossibilitiesUsingExistingSingles::in_squares(&mut sudoku);

        // Then the cell should not be changed, the value should be removed as a possibility from other cells in the
        // same square and all other cells should remain unchanged.
        assert_eq!(changed, true, "Sudoku template should have changed but was not.");
        assert_eq!(sudoku.cells[0][0].get_value(), 1, "The value at cell (0, 0) should be 1 but is not.");

        for (x, y) in iproduct!(0..9, 0..9) {
            if (x, y) == (0, 0) {
                continue
            } else if x < 3 && y < 3 {
                // For related cells:
                assert_eq!(
                    sudoku.cells[x][y].contains_possibility(1),
                    false,
                    "Cell at ({x}, {y}) still contains the value 1 as a possibility."
                );
            } else {
                // For unrelated cells:
                assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.");
            }
        }
    }

    #[test]
    fn in_squares_does_not_change_empty_sudoku() {
        // Given an empty sudoku.
        let mut sudoku = SudokuTemplate::from(EMPTY_SUDOKU.parse::<Sudoku>().unwrap());
        let original = sudoku.clone();

        // When I apply the strategy for rows and columns.
        let changed = EliminatePossibilitiesUsingExistingSingles::in_squares(&mut sudoku);

        // Then the sudoku should not have changed.
        assert_eq!(changed, false, "Sudoku template should not have changed.");
        assert_eq!(sudoku, original, "Sudoku template should not have changed.");
    }

    #[test]
    fn solve_correctly_returns_changed_flag() {
        // Given a sudoku with some filled cells.
        let mut sudoku = SudokuTemplate::from(SUDOKU.parse::<Sudoku>().unwrap());

        // When I apply the strategy using solve(), then it should return true.
        assert_eq!(EliminatePossibilitiesUsingExistingSingles {}.solve(&mut sudoku), true);

        // Given an empty sudoku.
        let mut empty_sudoku = SudokuTemplate::from(EMPTY_SUDOKU.parse::<Sudoku>().unwrap());

        // When I apply the strategy using solve(), then it should return false.
        assert_eq!(EliminatePossibilitiesUsingExistingSingles {}.solve(&mut empty_sudoku), false);
    }

    #[test]
    fn difficulty_is_easy() {
        assert_eq!(EliminatePossibilitiesUsingExistingSingles {}.difficulty(), Difficulty::Easy);
    }
}
