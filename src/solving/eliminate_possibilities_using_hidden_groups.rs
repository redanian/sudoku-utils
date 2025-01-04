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
/// This strategy iterates through all rows, columns, and blocks in a sudoku. It checks combinations of missing values
/// to identify hidden pairs, triples, or quads. When a hidden group is found, it eliminates other candidates from the
/// relevant cells.
pub(crate) struct EliminatePossibilitiesUsingHiddenCombinationsGroups;

impl EliminatePossibilitiesUsingHiddenCombinationsGroups {
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

impl SudokuSolvingStrategy for EliminatePossibilitiesUsingHiddenCombinationsGroups {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        EliminatePossibilitiesUsingHiddenCombinationsGroups::in_rows(sudoku) ||
            EliminatePossibilitiesUsingHiddenCombinationsGroups::in_columns(sudoku) ||
            EliminatePossibilitiesUsingHiddenCombinationsGroups::in_squares(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Medium
    }
}
