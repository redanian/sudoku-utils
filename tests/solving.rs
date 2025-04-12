mod examples;

use crate::examples::{EASY_SUDOKUS, HARD_SUDOKUS, MEDIUM_SUDOKUS};
use sudoku_utils::{solve, Sudoku};

fn assert_solved_correctly(sudoku: &str, solution: &str) {
    assert_eq!(
        solve(&sudoku.parse::<Sudoku>().unwrap()).to_string(),
        solution
    )
}

#[test]
fn solve_fn_correctly_solves_sudokus() {
    std::iter::empty()
        .chain(EASY_SUDOKUS.iter())
        .chain(MEDIUM_SUDOKUS.iter())
        .chain(HARD_SUDOKUS.iter())
        .for_each(|[sudoku, solution]| assert_solved_correctly(sudoku, solution))
}
