use std::cmp::min;

use itertools::{iproduct, Itertools};

use crate::solving::traits::SudokuTemplateTransformer;
use crate::traits::SudokuTemplate;

pub(crate) struct EliminatePossibilitiesUsingHiddenCombinationsGroups;

impl EliminatePossibilitiesUsingHiddenCombinationsGroups {
    fn in_rows(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        for row in 0..9 {
            let missing_values = sudoku.get_missing_values_in_row(row);
            for combination_len in 2..=min(missing_values.len(), 4) {
                for ref_combination in missing_values.iter().combinations(combination_len) {
                    let combination = &ref_combination.into_iter().map(|x| *x).collect_vec();
                    let containing_cells = (0..9)
                        .zip([row; 9])
                        .filter(|(y, x)| sudoku.cells[*x][*y].contains_any_possibilities(combination))
                        .map(|(y, x)| (x, y))
                        .collect_vec();
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

    fn in_columns(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        for column in 0..9 {
            let missing_values = sudoku.get_missing_values_in_column(column);
            for combination_len in 2..=min(missing_values.len(), 4) {
                for ref_combination in missing_values.iter().combinations(combination_len) {
                    let combination = &ref_combination.into_iter().map(|x| *x).collect_vec();
                    let containing_cells = (0..9)
                        .zip([column; 9])
                        .filter(|(x, y)| sudoku.cells[*x][*y].contains_any_possibilities(combination))
                        .collect_vec();
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

    fn in_squares(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        for (sq_row, sq_column) in iproduct!((0..3), (0..3)) {
            let missing_values = sudoku.get_missing_values_in_square(sq_row, sq_column);
            for combination_len in 2..=min(missing_values.len(), 4) {
                for ref_combination in missing_values.iter().combinations(combination_len) {
                    let combination = &ref_combination.into_iter().map(|x| *x).collect_vec();
                    let containing_cells = iproduct!((0..3), (0..3))
                        .map(|(x, y)| (3 * sq_row + x, 3 * sq_column + y))
                        .filter(|(x, y)| sudoku.cells[*x][*y].contains_any_possibilities(combination))
                        .collect_vec();
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

impl SudokuTemplateTransformer for EliminatePossibilitiesUsingHiddenCombinationsGroups {
    fn transform(&self, sudoku: &mut SudokuTemplate) -> bool {
        EliminatePossibilitiesUsingHiddenCombinationsGroups::in_rows(sudoku) ||
            EliminatePossibilitiesUsingHiddenCombinationsGroups::in_columns(sudoku) ||
            EliminatePossibilitiesUsingHiddenCombinationsGroups::in_squares(sudoku)
    }
}