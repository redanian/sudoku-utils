use crate::evaluating::functions::evaluate_difficulty;
use crate::solving::functions::{solve, solve_with_guessing};
use crate::traits::Difficulty;
use crate::traits::Sudoku;
use itertools::{iproduct, Itertools};
use rand::prelude::{SliceRandom, ThreadRng};
use rand::rng;

pub fn generate_sudoku() -> Sudoku {
    generate()
}

pub fn generate_sudoku_with_difficulty(difficulty: Difficulty) -> Sudoku {
    loop {
        let sudoku = generate();
        let difficulty_opt = evaluate_difficulty(&sudoku);

        if difficulty_opt == Some(difficulty) {
            return sudoku;
        }
    }
}

/// Generates an unsolved Sudoku with no difficulty guarantees.
fn generate() -> Sudoku {
    remove_cells(&generate_completed_sudoku(), &mut rng())
}

/// Generates a completed Sudoku.
fn generate_completed_sudoku() -> Sudoku {
    loop {
        let sudoku = solve_with_guessing(&Sudoku::empty());
        if sudoku.is_solved() {
            return sudoku;
        }
    }
}

fn remove_cells(sudoku: &Sudoku, random: &mut ThreadRng) -> Sudoku {
    let mut non_empty_cells = iproduct!(0..9, 0..9)
        .filter(|(x, y)| sudoku.cells[*x][*y] != 0)
        .collect_vec();
    non_empty_cells.shuffle(random);

    for cell in non_empty_cells {
        let new_sudoku = remove_cell(sudoku, cell);
        if solve(&new_sudoku).is_solved() {
            return remove_cells(&new_sudoku, random);
        }
    }

    sudoku.clone()
}

fn remove_cell(sudoku: &Sudoku, cell: (usize, usize)) -> Sudoku {
    let mut cells = sudoku.cells.clone();
    cells[cell.0][cell.1] = 0;
    Sudoku::new(cells)
}
