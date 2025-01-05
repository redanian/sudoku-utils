use std::cmp::min;

use itertools::{iproduct, Itertools};

use crate::solving::traits::{Difficulty, SudokuSolvingStrategy};
use crate::traits::SudokuTemplate;

/// Sudoku strategy that eliminates possibilities from empty cells by identifying hidden groups (hidden pairs, hidden
/// triples, hidden quads).
///
/// Consider the following example:
/// ```text
///    1 2 3 4 5 6 7 8 9
///    -----------------
/// A |2 5  |     |     |
/// B |     |4 1 9|     |
/// C |     |     |     |
///    -----------------
/// D |     |     |     |
/// E |     |     |5    |
/// F |     |     |2    |
///    -----------------
/// G |     |     |     |
/// H |     |     |     |
/// I |     |     |     |
///    -----------------
/// ```
/// The empty cells in row `B` have these possibilities:
/// - `B1`: [3, 6, 7, 8]
/// - `B2`: [3, 6, 7, 8]
/// - `B3`: [3 ,6, 7, 8]
/// - `B7`: [3, 6, 7, 8]
/// - `B8`: [2, 3, 5, 6, 7, 8]
/// - `B9`: [2, 3, 5, 6, 7, 8]
///
/// Although cells `B8` and `B9` have many possibilities, the numbers [2, 5] in row `B` can only exist in these cells.
/// The numbers [2, 5] are a hidden pair in these cells, so the other possibilities in these cells can be removed.
///
/// This strategy implementation iterates through all rows, columns, and blocks in a sudoku. It checks combinations of
/// missing values to identify hidden pairs, triples, or quads. When a hidden group is found, it eliminates other
/// candidates from the relevant cells.
pub(crate) struct EliminatePossibilitiesUsingHiddenGroups;

impl EliminatePossibilitiesUsingHiddenGroups {
    /// Eliminates possibilities from empty cells by identifying hidden groups and their containing cells in rows.
    fn in_rows(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // For each row
        for row in 0..9 {
            // Get all the missing values in the row.
            let missing_values = sudoku.get_missing_values_in_row(row);

            // For combination lengths [2, 3, 4]
            for combination_len in 2..=min(missing_values.len(), 4) {
                // Generate combinations of missing values.
                for ref_combination in missing_values.iter().combinations(combination_len) {
                    let combination = &ref_combination.into_iter().map(|x| *x).collect_vec();

                    // Find all empty cells that contain at least one value from the combination as a possibility.
                    let containing_cells = (0..9)
                        .zip([row; 9])
                        .filter(|(y, x)| sudoku.cells[*x][*y].is_empty())
                        .filter(|(y, x)| sudoku.cells[*x][*y].contains_any_possibilities(combination))
                        .map(|(y, x)| (x, y))
                        .collect_vec();

                    // If the number of containing cells matches the length of the combination, then it's a hidden
                    // group. Remove all other possibilities from these cells.
                    if combination_len == containing_cells.len() && combination_len != missing_values.len() {
                        for (x, y) in containing_cells {
                            made_changes |= sudoku.cells[x][y].remove_possibilities_outside_of(combination);
                        }
                    }
                }
            }
        }

        made_changes
    }

    /// Eliminates possibilities from empty cells by identifying hidden groups and their containing cells in columns.
    fn in_columns(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // For each column
        for column in 0..9 {
            // Get all the missing values in the column.
            let missing_values = sudoku.get_missing_values_in_column(column);

            // For combination lengths [2, 3, 4]
            for combination_len in 2..=min(missing_values.len(), 4) {
                // Generate combinations of missing values.
                for ref_combination in missing_values.iter().combinations(combination_len) {
                    let combination = &ref_combination.into_iter().map(|x| *x).collect_vec();

                    // Find all empty cells that contain at least one value from the combination as a possibility.
                    let containing_cells = (0..9)
                        .zip([column; 9])
                        .filter(|(x, y)| sudoku.cells[*x][*y].is_empty())
                        .filter(|(x, y)| sudoku.cells[*x][*y].contains_any_possibilities(combination))
                        .collect_vec();

                    // If the number of containing cells matches the length of the combination, then it's a hidden
                    // group. Remove all other possibilities from these cells.
                    if combination_len == containing_cells.len() && combination_len != missing_values.len() {
                        for (x, y) in containing_cells {
                            made_changes |= sudoku.cells[x][y].remove_possibilities_outside_of(combination);
                        }
                    }
                }
            }
        }

        made_changes
    }

    /// Eliminates possibilities from empty cells by identifying hidden groups and their containing cells in squares.
    fn in_squares(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // For each square
        for (sq_row, sq_column) in iproduct!(0..3, 0..3) {
            // Get all the missing values in the square.
            let missing_values = sudoku.get_missing_values_in_square(sq_row, sq_column);

            // For combination lengths [2, 3, 4]
            for combination_len in 2..=min(missing_values.len(), 4) {
                // Generate combinations of missing values.
                for ref_combination in missing_values.iter().combinations(combination_len) {
                    let combination = &ref_combination.into_iter().map(|x| *x).collect_vec();

                    // Find all empty cells that contain at least one value from the combination as a possibility.
                    let containing_cells = iproduct!(0..3, 0..3)
                        .map(|(x, y)| (3 * sq_row + x, 3 * sq_column + y))
                        .filter(|(x, y)| sudoku.cells[*x][*y].contains_any_possibilities(combination))
                        .collect_vec();

                    // If the number of containing cells matches the length of the combination, then it's a hidden
                    // group. Remove all other possibilities from these cells.
                    if combination_len == containing_cells.len() && combination_len != missing_values.len() {
                        for (x, y) in containing_cells {
                            made_changes |= sudoku.cells[x][y].remove_possibilities_outside_of(combination);
                        }
                    }
                }
            }
        }

        made_changes
    }
}

impl SudokuSolvingStrategy for EliminatePossibilitiesUsingHiddenGroups {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        EliminatePossibilitiesUsingHiddenGroups::in_rows(sudoku) ||
            EliminatePossibilitiesUsingHiddenGroups::in_columns(sudoku) ||
            EliminatePossibilitiesUsingHiddenGroups::in_squares(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
}

#[cfg(test)]
mod tests {
    use crate::solving::eliminate_possibilities_using_existing_singles::EliminatePossibilitiesUsingExistingSingles;
    use crate::solving::traits::SudokuSolvingStrategy;
    use crate::traits::SudokuTemplate;
    use crate::Sudoku;

    const SUDOKU_WITHOUT_HIDDEN_GROUP: &str = "\
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

    /// Cleans up a sudoku template by removing possibilities of empty cells by using existing values.
    fn clean(sudoku: &mut SudokuTemplate) {
        let cleaner = EliminatePossibilitiesUsingExistingSingles {};
        while cleaner.solve(sudoku) {}
    }

    fn as_template(string: &str) -> SudokuTemplate {
        let mut sudoku = SudokuTemplate::from(string.parse::<Sudoku>().unwrap());
        clean(&mut sudoku);
        sudoku
    }

    mod in_rows {
        use crate::solving::eliminate_possibilities_using_hidden_groups::tests::{as_template, SUDOKU_WITHOUT_HIDDEN_GROUP};
        use crate::solving::eliminate_possibilities_using_hidden_groups::EliminatePossibilitiesUsingHiddenGroups;
        use itertools::iproduct;

        const SUDOKU_WITH_HIDDEN_PAIR_IN_ROW: &str = "\
        ..34567..\
        89.......\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        ";

        const SUDOKU_WITH_HIDDEN_TRIPLE_IN_ROW: &str = "\
        ...456...\
        78.......\
        ..9......\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        ";

        const SUDOKU_WITH_HIDDEN_QUAD_IN_ROW: &str = "\
        ...45....\
        78.......\
        .69......\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        ";

        #[test]
        fn in_rows_correctly_processes_hidden_pair() {
            // Given a sudoku with the hidden pair (8, 9) in the last two cells of the first row.
            let mut sudoku = as_template(SUDOKU_WITH_HIDDEN_PAIR_IN_ROW);
            let original = sudoku.clone();

            // When the strategy is applied for rows.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_rows(&mut sudoku);

            // Then the last two cells of the first row should only contain 8 and 9 as possible values and other cells
            // in the sudoku should remain unchanged.
            assert_eq!(changed, true, "Sudoku template should have changed but was not.");
            assert_ne!(original, sudoku, "Sudoku template should have changed but was not.");

            for (x, y) in iproduct!(0..9, 0..9) {
                if (x, y) == (0, 7) || (x, y) == (0, 8) {
                    // For the cells containing the hidden group:
                    assert_eq!(
                        sudoku.cells[x][y].possible_values(),
                        vec![8, 9],
                        "Cell at ({x}, {y}) does not contain only [8, 9] as possible values."
                    );
                } else {
                    // For all other cells:
                    assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.")
                }
            }
        }

        #[test]
        fn in_rows_correctly_processes_hidden_triple() {
            // Given a sudoku with the hidden triple (7, 8, 9) in the last three cells of the first row.
            let mut sudoku = as_template(SUDOKU_WITH_HIDDEN_TRIPLE_IN_ROW);
            let original = sudoku.clone();

            // When the strategy is applied for rows.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_rows(&mut sudoku);

            // Then the last three cells of the first row should only contain 7, 8 and 9 as possible values and other
            // cells in the sudoku should remain unchanged.
            assert_eq!(changed, true, "Sudoku template should have changed but was not.");
            assert_ne!(original, sudoku, "Sudoku template should have changed but was not.");

            for (x, y) in iproduct!(0..9, 0..9) {
                if (x, y) == (0, 6) || (x, y) == (0, 7) || (x, y) == (0, 8) {
                    // For the cells containing the hidden group:
                    assert_eq!(
                        sudoku.cells[x][y].possible_values(),
                        vec![7, 8, 9],
                        "Cell at ({x}, {y}) does not contain only [7, 8, 9] as possible values."
                    );
                } else {
                    // For all other cells:
                    assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.")
                }
            }
        }

        #[test]
        fn in_rows_correctly_processes_hidden_quad() {
            // Given a sudoku with the hidden quad (6, 7, 8, 9) in the last four cells of the first row.
            let mut sudoku = as_template(SUDOKU_WITH_HIDDEN_QUAD_IN_ROW);
            let original = sudoku.clone();

            // When the strategy is applied for rows.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_rows(&mut sudoku);

            // Then the last four cells of the first row should only contain 6, 7, 8 and 9 as possible values and other
            // cells in the sudoku should remain unchanged.
            assert_eq!(changed, true, "Sudoku template should have changed but was not.");
            assert_ne!(original, sudoku, "Sudoku template should have changed but was not.");

            for (x, y) in iproduct!(0..9, 0..9) {
                if (x, y) == (0, 5) || (x, y) == (0, 6) || (x, y) == (0, 7) || (x, y) == (0, 8) {
                    // For the cells containing the hidden group:
                    assert_eq!(
                        sudoku.cells[x][y].possible_values(),
                        vec![6, 7, 8, 9],
                        "Cell at ({x}, {y}) does not contain only [6, 7, 8, 9] as possible values."
                    );
                } else {
                    // For all other cells:
                    assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.")
                }
            }
        }

        #[test]
        fn in_rows_does_not_change_sudoku_without_hidden_groups() {
            // Given a sudoku without hidden groups.
            let mut sudoku = as_template(SUDOKU_WITHOUT_HIDDEN_GROUP);
            let original = sudoku.clone();

            // When the strategy is applied for rows.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_rows(&mut sudoku);

            // Then the sudoku should not change.
            assert_eq!(changed, false, "Sudoku template should not have changed.");
            assert_eq!(sudoku, original, "Sudoku template should not have changed.");
        }
    }

    mod in_columns {
        use crate::solving::eliminate_possibilities_using_hidden_groups::tests::{as_template, SUDOKU_WITHOUT_HIDDEN_GROUP};
        use crate::solving::eliminate_possibilities_using_hidden_groups::EliminatePossibilitiesUsingHiddenGroups;
        use itertools::iproduct;

        const SUDOKU_WITH_HIDDEN_PAIR_IN_COLUMN: &str = "\
        .89......\
        .........\
        3........\
        4........\
        5........\
        6........\
        7........\
        .........\
        .........\
        ";

        const SUDOKU_WITH_HIDDEN_TRIPLE_IN_COLUMN: &str = "\
        ..7......\
        .8.......\
        ..9......\
        4........\
        5........\
        6........\
        .........\
        .........\
        .........\
        ";

        const SUDOKU_WITH_HIDDEN_QUAD_IN_COLUMN: &str = "\
        .7.......\
        .86......\
        ..9......\
        4........\
        5........\
        .........\
        .........\
        .........\
        .........\
        ";

        #[test]
        fn in_columns_correctly_processes_hidden_pair() {
            // Given a sudoku with the hidden pair (8, 9) in the last two cells of the first column.
            let mut sudoku = as_template(SUDOKU_WITH_HIDDEN_PAIR_IN_COLUMN);
            let original = sudoku.clone();

            // When the strategy is applied for columns.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_columns(&mut sudoku);

            // Then the last two cells of the first column should only contain 8 and 9 as possible values and other
            // cells in the sudoku should remain unchanged.
            assert_eq!(changed, true, "Sudoku template should have changed but was not.");
            assert_ne!(original, sudoku, "Sudoku template should have changed but was not.");

            for (x, y) in iproduct!(0..9, 0..9) {
                if (x, y) == (7, 0) || (x, y) == (8, 0) {
                    // For the cells containing the hidden group:
                    assert_eq!(
                        sudoku.cells[x][y].possible_values(),
                        vec![8, 9],
                        "Cell at ({x}, {y}) does not contain only [8, 9] as possible values."
                    );
                } else {
                    // For all other cells:
                    assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.")
                }
            }
        }

        #[test]
        fn in_columns_correctly_processes_hidden_triple() {
            // Given a sudoku with the hidden triple (7, 8, 9) in the last three cells of the first column.
            let mut sudoku = as_template(SUDOKU_WITH_HIDDEN_TRIPLE_IN_COLUMN);
            let original = sudoku.clone();

            // When the strategy is applied for columns.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_columns(&mut sudoku);

            // Then the last three cells of the first column should only contain 7, 8 and 9 as possible values and other
            // cells in the sudoku should remain unchanged.
            assert_eq!(changed, true, "Sudoku template should have changed but was not.");
            assert_ne!(original, sudoku, "Sudoku template should have changed but was not.");

            for (x, y) in iproduct!(0..9, 0..9) {
                if (x, y) == (6, 0) || (x, y) == (7, 0) || (x, y) == (8, 0) {
                    // For the cells containing the hidden group:
                    assert_eq!(
                        sudoku.cells[x][y].possible_values(),
                        vec![7, 8, 9],
                        "Cell at ({x}, {y}) does not contain only [7, 8, 9] as possible values."
                    );
                } else {
                    // For all other cells:
                    assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.")
                }
            }
        }

        #[test]
        fn in_columns_correctly_processes_hidden_quad() {
            // Given a sudoku with the hidden quad (6, 7, 8, 9) in the last four cells of the first column.
            let mut sudoku = as_template(SUDOKU_WITH_HIDDEN_QUAD_IN_COLUMN);
            let original = sudoku.clone();

            // When the strategy is applied for columns.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_columns(&mut sudoku);

            // Then the last four cells of the first column should only contain 6, 7, 8 and 9 as possible values and
            // other cells in the sudoku should remain unchanged.
            assert_eq!(changed, true, "Sudoku template should have changed but was not.");
            assert_ne!(original, sudoku, "Sudoku template should have changed but was not.");

            for (x, y) in iproduct!(0..9, 0..9) {
                if (x, y) == (5, 0) || (x, y) == (6, 0) || (x, y) == (7, 0) || (x, y) == (8, 0) {
                    // For the cells containing the hidden group:
                    assert_eq!(
                        sudoku.cells[x][y].possible_values(),
                        vec![6, 7, 8, 9],
                        "Cell at ({x}, {y}) does not contain only [6, 7, 8, 9] as possible values."
                    );
                } else {
                    // For all other cells:
                    assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.")
                }
            }
        }

        #[test]
        fn in_columns_does_not_change_sudoku_without_hidden_groups() {
            // Given a sudoku without hidden groups.
            let mut sudoku = as_template(SUDOKU_WITHOUT_HIDDEN_GROUP);
            let original = sudoku.clone();

            // When the strategy is applied for columns.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_columns(&mut sudoku);

            // Then the sudoku should not change.
            assert_eq!(changed, false, "Sudoku template should not have changed.");
            assert_eq!(sudoku, original, "Sudoku template should not have changed.");
        }
    }

    mod in_squares {
        use crate::solving::eliminate_possibilities_using_hidden_groups::tests::{as_template, SUDOKU_WITHOUT_HIDDEN_GROUP};
        use crate::solving::eliminate_possibilities_using_hidden_groups::EliminatePossibilitiesUsingHiddenGroups;
        use itertools::iproduct;

        const SUDOKU_WITH_HIDDEN_PAIR_IN_SQUARE: &str = "\
        ..3....89\
        456......\
        7........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        ";

        const SUDOKU_WITH_HIDDEN_TRIPLE_IN_SQUARE: &str = "\
        .....789.\
        456......\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        ";

        const SUDOKU_WITH_HIDDEN_QUAD_IN_SQUARE: &str = "\
        ....6789.\
        45.......\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        .........\
        ";

        #[test]
        fn in_squares_correctly_processes_hidden_pair() {
            // Given a sudoku with the hidden pair (8, 9) in the last two cells of the first square.
            let mut sudoku = as_template(SUDOKU_WITH_HIDDEN_PAIR_IN_SQUARE);
            let original = sudoku.clone();

            // When the strategy is applied for squares.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_squares(&mut sudoku);

            // Then the last two cells of the first square should only contain 8 and 9 as possible values and other
            // cells in the sudoku should remain unchanged.
            assert_eq!(changed, true, "Sudoku template should have changed but was not.");
            assert_ne!(original, sudoku, "Sudoku template should have changed but was not.");

            for (x, y) in iproduct!(0..9, 0..9) {
                if (x, y) == (2, 1) || (x, y) == (2, 2) {
                    // For the cells containing the hidden group:
                    assert_eq!(
                        sudoku.cells[x][y].possible_values(),
                        vec![8, 9],
                        "Cell at ({x}, {y}) does not contain only [8, 9] as possible values."
                    );
                } else {
                    // For all other cells:
                    assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.")
                }
            }
        }

        #[test]
        fn in_squares_correctly_processes_hidden_triple() {
            // Given a sudoku with the hidden triple (7, 8, 9) in the last three cells of the first square.
            let mut sudoku = as_template(SUDOKU_WITH_HIDDEN_TRIPLE_IN_SQUARE);
            let original = sudoku.clone();

            // When the strategy is applied for squares.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_squares(&mut sudoku);

            // Then the last three cells of the first square should only contain 7, 8 and 9 as possible values and other
            // cells in the sudoku should remain unchanged.
            assert_eq!(changed, true, "Sudoku template should have changed but was not.");
            assert_ne!(original, sudoku, "Sudoku template should have changed but was not.");

            for (x, y) in iproduct!(0..9, 0..9) {
                if (x, y) == (2, 0) || (x, y) == (2, 1) || (x, y) == (2, 2) {
                    // For the cells containing the hidden group:
                    assert_eq!(
                        sudoku.cells[x][y].possible_values(),
                        vec![7, 8, 9],
                        "Cell at ({x}, {y}) does not contain only [7, 8, 9] as possible values."
                    );
                } else {
                    // For all other cells:
                    assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.")
                }
            }
        }

        #[test]
        fn in_squares_correctly_processes_hidden_quad() {
            // Given a sudoku with the hidden quad (6, 7, 8, 9) in the last four cells of the first square.
            let mut sudoku = as_template(SUDOKU_WITH_HIDDEN_QUAD_IN_SQUARE);
            let original = sudoku.clone();

            // When the strategy is applied for squares.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_squares(&mut sudoku);

            // Then the last four cells of the first square should only contain 6, 7, 8 and 9 as possible values and
            // other cells in the sudoku should remain unchanged.
            assert_eq!(changed, true, "Sudoku template should have changed but was not.");
            assert_ne!(original, sudoku, "Sudoku template should have changed but was not.");

            for (x, y) in iproduct!(0..9, 0..9) {
                if (x, y) == (1, 2) || (x, y) == (2, 0) || (x, y) == (2, 1) || (x, y) == (2, 2) {
                    // For the cells containing the hidden group:
                    assert_eq!(
                        sudoku.cells[x][y].possible_values(),
                        vec![6, 7, 8, 9],
                        "Cell at ({x}, {y}) does not contain only [6, 7, 8, 9] as possible values."
                    );
                } else {
                    // For all other cells:
                    assert_eq!(sudoku.cells[x][y], original.cells[x][y], "Cell at ({x}, {y}) was changed.")
                }
            }
        }

        #[test]
        fn in_squares_does_not_change_sudoku_without_hidden_groups() {
            // Given a sudoku without hidden groups.
            let mut sudoku = as_template(SUDOKU_WITHOUT_HIDDEN_GROUP);
            let original = sudoku.clone();

            // When the strategy is applied for squares.
            let changed = EliminatePossibilitiesUsingHiddenGroups::in_squares(&mut sudoku);

            // Then the sudoku should not change.
            assert_eq!(changed, false, "Sudoku template should not have changed.");
            assert_eq!(sudoku, original, "Sudoku template should not have changed.");
        }
    }
}
