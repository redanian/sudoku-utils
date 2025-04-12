use crate::solving::functions::solve_with_difficulty;
use crate::traits::Difficulty;
use crate::traits::Sudoku;

/// Evaluates the difficulty level of the provided Sudoku by trying to solve it. Returns: `Some(difficulty)` if the
/// puzzle can be solved using logical strategies without guessing, or`None` if the puzzle cannot not be solved.
pub fn evaluate_difficulty(sudoku: &Sudoku) -> Option<Difficulty> {
    for difficulty in Difficulty::VALUES {
        if solve_with_difficulty(&sudoku, difficulty).is_solved() {
            return Some(difficulty);
        }
    }

    None
}
