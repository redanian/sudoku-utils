use crate::traits::SudokuTemplate;

/// Defines a solving strategy for a `SudokuTemplate`. Implementors of this trait provide specific algorithms or
/// techniques to manipulate and potentially solve a sudoku.
pub(crate) trait SudokuSolvingStrategy {
    /// Applies a solving technique to a `SudokuTemplate`. Returns `true` if any changes were made to the `sudoku`,
    /// indicating that further changes might be required.
    ///
    /// # Arguments
    ///
    /// * `sudoku` - A mutable reference to a `SudokuTemplate` to be solved.
    ///
    /// # Returns
    ///
    /// `bool` - `true` if the `sudoku` was modified, `false` otherwise.
    fn solve(&self, sudoku: &mut SudokuTemplate) -> bool;

    /// Provides the difficulty level of the implemented strategy.
    ///
    /// # Returns
    ///
    /// `Difficulty` - the difficulty level of the strategy.
    fn difficulty(&self) -> Difficulty;
}

/// Difficulty levels of sudoku solving strategies.
pub(crate) enum Difficulty {
    Easy,
    Medium,
    Hard,
}
