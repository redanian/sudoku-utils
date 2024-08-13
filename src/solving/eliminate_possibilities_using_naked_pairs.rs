use itertools::{iproduct, Itertools};

use crate::solving::traits::{Difficulty, SudokuSolvingStrategy};
use crate::traits::SudokuTemplate;

pub(crate) struct EliminatePossibilitiesUsingNakedPairs;

impl EliminatePossibilitiesUsingNakedPairs {
    fn in_rows_and_columns(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // For each row or columns
        for x in 0..9 {
            // Get existing values
            let values_in_row = sudoku.get_values_in_row(x);
            let values_in_column = sudoku.get_values_in_column(x);

            // Calculate missing values
            let missing_values_in_row = &(1..=9).filter(|&n| !values_in_row.contains(&n)).collect_vec();
            let missing_values_in_column = &(1..=9).filter(|&n| !values_in_column.contains(&n)).collect_vec();

            // For each pair of missing values in the row
            for (&n1, &n2) in iproduct!(missing_values_in_row, missing_values_in_row) {
                // Skip duplicate pairs
                if n1 != n2 && n1 < n2 {
                    // Get the column numbers of the empty cells that contain as a possibility only this pair
                    let columns = (0..9)
                        .filter(|&y| sudoku.cells[x][y].is_empty())
                        .filter(|&y| sudoku.cells[x][y].possible_values().iter().all(|&v| v == n1 || v == n2))
                        .collect_vec();
                    // If there are only two cells that contain only the pair as possible values
                    if columns.len() == 2 {
                        // Remove the pair as possibility from other cells in the row
                        (0..9)
                            .filter(|y| !columns.contains(y))
                            .for_each(|y| {
                                made_changes |= sudoku.cells[x][y].remove_possibility(n1);
                                made_changes |= sudoku.cells[x][y].remove_possibility(n2);
                            });
                    }
                }
            }

            // For each pair of missing values in the column
            for (&n1, &n2) in iproduct!(missing_values_in_column, missing_values_in_column) {
                // Skip duplicate pairs
                if n1 != n2 && n1 < n2 {
                    // Get the row numbers of the empty cells that contain as a possibility only this pair
                    let columns = (0..9)
                        .filter(|&y| sudoku.cells[y][x].is_empty())
                        .filter(|&y| sudoku.cells[y][x].possible_values().iter().all(|&v| v == n1 || v == n2))
                        .collect_vec();
                    // If there are only two cells that contain only the pair as possible values
                    if columns.len() == 2 {
                        // Remove the pair as possibility from other cells in the row
                        (0..9)
                            .filter(|y| !columns.contains(y))
                            .for_each(|y| {
                                made_changes |= sudoku.cells[y][x].remove_possibility(n1);
                                made_changes |= sudoku.cells[y][x].remove_possibility(n2);
                            });
                    }
                }
            }
        }

        made_changes
    }
}

impl SudokuSolvingStrategy for EliminatePossibilitiesUsingNakedPairs {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        EliminatePossibilitiesUsingNakedPairs::in_rows_and_columns(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
}
