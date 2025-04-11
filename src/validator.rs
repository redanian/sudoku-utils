use std::ops::Not;

use crate::traits::Sudoku;

pub(crate) fn is_valid(sudoku: &Sudoku) -> bool {
    sudoku.get_cells()
        .into_iter()
        .flatten()
        .any(|x| *x > 9)
        .not()
}
