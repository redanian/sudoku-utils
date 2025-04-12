use crate::solving::traits::SudokuSolvingStrategy;
use crate::traits::Difficulty;
use crate::traits::SudokuTemplate;
use itertools::iproduct;

pub(crate) struct EliminatePossibilitiesUsingYWing;

impl EliminatePossibilitiesUsingYWing {
    fn everywhere(sudoku: &mut SudokuTemplate) -> bool {
        let mut made_changes = false;

        // Find the first wing (cell that contains only two possible values).
        for first_wing in iproduct!(0..9, 0..9) {
            let first_wing_possible_values = sudoku.cells[first_wing.0][first_wing.1].possible_values();
            if first_wing_possible_values.len() != 2 {
                continue;
            }
            // Find the second wing (cell that is unrelated to the first wing, has only two possible values and has
            // exactly one common possible value with the first wing).
            for second_wing in iproduct!(0..9, 0..9) {
                if Self::are_cells_related(first_wing, second_wing) {
                    continue;
                }
                let second_wing_possible_values = sudoku.cells[second_wing.0][second_wing.1].possible_values();
                if second_wing_possible_values.len() != 2 {
                    continue;
                }
                if !Self::have_only_one_common_element(&first_wing_possible_values, &second_wing_possible_values, ) {
                    continue;
                }

                let common_candidate =
                    Self::get_common_element(&first_wing_possible_values, &second_wing_possible_values);
                let distinct_candidates = (
                    *first_wing_possible_values.iter().find(|&&x| x != common_candidate).unwrap(),
                    *second_wing_possible_values.iter().find(|&&x| x != common_candidate).unwrap(),
                );

                // Find the middle (cell that is related to both wings, has only two possible values and the possible
                // values are the distinct candidates of the wings).
                for middle in iproduct!(0..9, 0..9) {
                    if !Self::are_cells_related(first_wing, middle) || !Self::are_cells_related(second_wing, middle) {
                        continue;
                    }
                    let middle_possible_values = sudoku.cells[middle.0][middle.1].possible_values();
                    if middle_possible_values.len() != 2 {
                        continue
                    }
                    if middle_possible_values
                        .iter()
                        .any(|&x| x != distinct_candidates.0 && x != distinct_candidates.1) {
                        continue
                    }

                    // Now we have found a Y Wing pattern. The common candidate of the wings can only be placed in one
                    // of the wings and cannot be placed in any of the cells that are related to both wings. This means
                    // that the common candidate can be removed as a possibility to all cells that are related to both
                    // wings.
                    for related in iproduct!(0..9, 0..9) {
                        if !Self::are_cells_related(first_wing, related) ||
                            !Self::are_cells_related(second_wing, related) {
                            continue;
                        }

                        made_changes |= sudoku.cells[related.0][related.1].remove_possibility(common_candidate);
                    }
                }
            }
        }

        made_changes
    }

    fn are_cells_related(first: (usize, usize), second: (usize, usize)) -> bool {
        first.0 == second.0
            || first.1 == second.1
            || (first.0 / 3 == second.0 / 3 && first.1 / 3 == second.1 / 3)
    }

    fn have_only_one_common_element(first: &Vec<usize>, second: &Vec<usize>) -> bool {
        (first[0] == second[0] && first[1] != second[1])
            || (first[0] == second[1] && first[1] != second[0])
            || (first[1] == second[0] && first[0] != second[1])
            || (first[1] == second[1] && first[0] != second[0])
    }

    fn get_common_element(first: &Vec<usize>, second: &Vec<usize>) -> usize {
        if first[0] == second[0] || first[0] == second[1] {
            first[0]
        } else if first[1] == second[0] || first[1] == second[1] {
            first[1]
        } else {
            panic!("Vectors {first:?} and {second:?} have no common element.")
        }
    }
}

impl SudokuSolvingStrategy for EliminatePossibilitiesUsingYWing {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        EliminatePossibilitiesUsingYWing::everywhere(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Hard
    }
}