use crate::evaluating::functions::evaluate_difficulty;
use crate::solving::functions::{solve, solve_with_guessing, solve_with_statistics};
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
        // Generate unsolved sudoku.
        let sudoku = generate();

        // Determine sudoku difficulty.
        let difficulty_opt = evaluate_difficulty(&sudoku);

        // If the puzzle difficulty matches the requested difficulty.
        if difficulty_opt == Some(difficulty) {
            // Get puzzle solving statistics by solving it.
            let (_, statistics) = solve_with_statistics(&sudoku);

            // Check if all implemented strategies that are associated with the requested difficulty were used at least
            // once during solving so that the puzzle is more interesting.
            let all_difficulty_level_strategies_used = statistics
                .iter()
                .filter(|(_, (strategy_difficulty, count))| *strategy_difficulty == difficulty)
                .map(|(_, (_, count))| *count)
                .all(|count| count > 0);

            // Return only if all implemented strategies with the requested difficulty were used at least once.
            if all_difficulty_level_strategies_used {
                return sudoku;
            }
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
