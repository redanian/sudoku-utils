use itertools::Itertools;

use crate::utils::BoolIteratorUtils;

/// Represents a modifiable sudoku cell.
pub(crate) struct Cell {
    value: usize,
    possibilities: [bool; 9],
}

impl Cell {
    pub(crate) fn new(value: usize) -> Cell {
        // Validate value.
        let safe_value = if value > 9 { 0 } else { value };

        // Create cell.
        Cell {
            value: safe_value,
            possibilities: Cell::gen_possibilities(safe_value),
        }
    }

    fn gen_possibilities(value: usize) -> [bool; 9] {
        (1..=9)
            .map(|i| value == 0 || i == value)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or([false; 9])
    }

    /// Return the value of the cell.
    pub(crate) fn get_value(&self) -> usize {
        self.value
    }

    /// Returns `true` if the value of the cell is not set, `false` otherwise.
    pub(crate) fn is_empty(&self) -> bool {
        self.value == 0
    }

    /// Returns `true` if the value of the cell is set, `false` otherwise.
    pub(crate) fn is_set(&self) -> bool {
        !self.is_empty()
    }

    /// Provides the possible values that can be set.
    pub(crate) fn possible_values(&self) -> Vec<usize> {
        (1..=9)
            .filter(|i| self.possibilities[i - 1])
            .collect()
    }

    pub(crate) fn contains_possibility(&self, possibility: usize) -> bool {
        self.possibilities[possibility - 1]
    }

    pub(crate) fn contains_any_possibilities(&self, possibilities: &[usize]) -> bool {
        possibilities
            .iter()
            .map(|value| self.possibilities[value - 1])
            .any_true()
    }

    /// Removes a specified value from the cell's possibilities. If as a result only one possible value is left, it will
    /// be set as the cell's value. Returns `true` if the cell state changed as a result of this operation, or `false`
    /// otherwise.
    pub(crate) fn remove_possibility(&mut self, value: usize) -> bool {
        // Check if the value is valid and still possible.
        if value < 1 || value > 9 || !self.possibilities[value - 1] {
            return false;
        }

        // Remove value from the possibilities.
        self.possibilities[value - 1] = false;

        // Check if only one possible value remains.
        let remaining_possibilities = self.possible_values();
        if remaining_possibilities.len() == 1 {
            self.value = remaining_possibilities[0];
        }
        true
    }

    pub(crate) fn remove_possibilities(&mut self, possibilities: &[usize]) -> bool {
        possibilities
            .iter()
            .map(|&possibility| self.remove_possibility(possibility))
            .any_true_exhaustive()
    }

    pub(crate) fn remove_possibilities_outside_of(&mut self, possibilities: &[usize]) -> bool {
        (1..=9)
            .filter(|n| !possibilities.contains(n))
            .map(|n| self.remove_possibility(n))
            .any_true_exhaustive()
    }


    /// Sets the value of the cell and removes all other possibilities. Returns `true` if the cell state changed as a
    /// result of this operation, or `false` otherwise.
    pub(crate) fn set_value(&mut self, value: usize) -> bool {
        // Check if the new value is valid and different from the current value.
        if value < 1 || value > 9 || value == self.value {
            return false;
        }

        // Set the value and update the possibilities array.
        self.value = value;
        self.possibilities = Cell::gen_possibilities(value);
        true
    }
}
