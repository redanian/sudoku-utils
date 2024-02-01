use crate::traits::SudokuTemplate;

/// Defines a transformation strategy for a `SudokuTemplate`. Implementors of this trait provide specific algorithms or
/// techniques to manipulate and potentially solve a sudoku.
pub(crate) trait SudokuTemplateTransformer {
    /// Applies a specific transformation or solving technique to a `SudokuTemplate`. Returns `true` if any changes were
    /// made to the `sudoku`, indicating that further transformations might be required.
    ///
    /// # Arguments
    ///
    /// * `sudoku` - A mutable reference to a `SudokuTemplate` to be transformed.
    ///
    /// # Returns
    ///
    /// `bool` - `true` if the `sudoku` was modified, `false` otherwise.
    fn transform(&self, sudoku: &mut SudokuTemplate) -> bool;
}
