use crate::solving::traits::SudokuSolvingStrategy;
use crate::traits::Difficulty;
use crate::traits::SudokuTemplate;
use rand::prelude::IteratorRandom;
use rand::rng;

pub(crate) struct SetSingleRandomly;

impl SetSingleRandomly {
    fn make_a_guess(sudoku: &mut SudokuTemplate) -> bool {
        let rnd = &mut rng();

        let empty_cell_optional = sudoku.empty_cells_mut().choose(rnd);
        if empty_cell_optional.is_none() {
            return false
        }
        let empty_cell = empty_cell_optional.unwrap();

        let value_opt = empty_cell.possible_values().into_iter().choose(rnd);
        if value_opt.is_none() {
            return false
        }
        let value = value_opt.unwrap();

        empty_cell.set_value(value);
        true
    }
}

impl SudokuSolvingStrategy for SetSingleRandomly {
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool {
        SetSingleRandomly::make_a_guess(sudoku)
    }

    fn difficulty(&self) -> Difficulty {
        Difficulty::Easy
    }
}