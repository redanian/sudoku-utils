use crate::solving::traits::SudokuSolvingStrategy;
use crate::traits::Difficulty;
use crate::traits::SudokuTemplate;
use itertools::Itertools;

pub(crate) struct EliminatePossibilitiesUsingXWing;

impl EliminatePossibilitiesUsingXWing {
    fn in_rows(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // Find the first row and the value of the wing (row where the value is a candidate in only two cells).
        for first_row in 0..9 {
            for value in sudoku.get_missing_values_in_row(first_row) {
                let columns = (0..9)
                    .into_iter()
                    .filter(|&col| sudoku.cells[first_row][col].contains_possibility(value))
                    .collect_vec();
                if columns.len() != 2 {
                    continue;
                }

                // Find the second row of wing (row where the value is also a candidate only in the two cells that are
                // in the same column as the cells in the first row).
                for second_row in first_row..9 {
                    if first_row == second_row {
                        continue;
                    }
                    let columns_in_second_row = (0..9)
                        .into_iter()
                        .filter(|&col| sudoku.cells[second_row][col].contains_possibility(value))
                        .collect_vec();
                    if columns != columns_in_second_row {
                        continue;
                    }

                    // Now we have found an X Wing pattern. For the provided columns, the value is a candidate only in
                    // the cells that are part of the two provided rows. In all the other cells of these columns, the
                    // value is not possible and can be removed as a candidate.

                    for other_row in 0..9 {
                        if other_row == first_row || other_row == second_row {
                            continue;
                        }

                        let &first_column = columns.get(0).unwrap();
                        let &second_column = columns.get(1).unwrap();

                        made_changes |= sudoku.cells[other_row][first_column].remove_possibility(value);
                        made_changes |= sudoku.cells[other_row][second_column].remove_possibility(value);
                    }
                }
            }
        }

        made_changes
    }
}

impl SudokuSolvingStrategy for EliminatePossibilitiesUsingXWing {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        EliminatePossibilitiesUsingXWing::in_rows(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }

    fn name(&self) -> &'static str {
        "EliminatePossibilitiesByDetectingXWingPatterns"
    }
}