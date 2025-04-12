use itertools::{iproduct, Itertools};

use crate::traits::cell::Cell;
use crate::traits::sudoku::Sudoku;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct SudokuTemplate {
    pub(crate) cells: [[Cell; 9]; 9],
}

impl SudokuTemplate {

    pub(crate) fn empty_cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .filter(|cell| cell.is_empty())
    }

    pub(crate) fn get_values_in_row(&self, row: usize) -> Vec<usize> {
        self.cells[row]
            .iter()
            .filter(|&cell| cell.is_set())
            .map(|cell| cell.get_value())
            .collect_vec()
    }

    pub(crate) fn get_values_in_column(&self, column: usize) -> Vec<usize> {
        (0..9)
            .map(|row| &self.cells[row][column])
            .filter(|&cell| cell.is_set())
            .map(|cell| cell.get_value())
            .collect_vec()
    }

    pub(crate) fn get_values_in_square(&self, row: usize, column: usize) -> Vec<usize> {
        iproduct!(0..3, 0..3)
            .map(|(x, y)| &self.cells[3 * row + x][3 * column + y])
            .filter(|&cell| cell.is_set())
            .map(|cell| cell.get_value())
            .collect_vec()
    }

    fn get_missing_values(from: &[usize]) -> Vec<usize> {
        (1..=9).filter(|value| !from.contains(value)).collect_vec()
    }

    pub(crate) fn get_missing_values_in_row(&self, row: usize) -> Vec<usize> {
        SudokuTemplate::get_missing_values(&self.get_values_in_row(row))
    }

    pub(crate) fn get_missing_values_in_column(&self, column: usize) -> Vec<usize> {
        SudokuTemplate::get_missing_values(&self.get_values_in_column(column))
    }

    pub(crate) fn get_missing_values_in_square(&self, row: usize, column: usize) -> Vec<usize> {
        SudokuTemplate::get_missing_values(&self.get_values_in_square(row, column))
    }
}

impl From<Sudoku> for SudokuTemplate {
    fn from(sudoku: Sudoku) -> SudokuTemplate {
        let cells: [[Cell; 9]; 9] = sudoku.get_cells().map(|row| row.map(|n| Cell::new(n)));

        SudokuTemplate {
            cells
        }
    }
}