use crate::solving::strategies::eliminate_possibilities_using_existing_singles::EliminatePossibilitiesUsingExistingSingles;
use crate::solving::strategies::eliminate_possibilities_using_hidden_groups::EliminatePossibilitiesUsingHiddenGroups;
use crate::solving::strategies::eliminate_possibilities_using_naked_pairs::EliminatePossibilitiesUsingNakedPairs;
use crate::solving::strategies::eliminate_possibilities_using_pointing::EliminatePossibilitiesUsingPointing;
use crate::solving::strategies::eliminate_possibilities_using_x_wing::EliminatePossibilitiesUsingXWing;
use crate::solving::strategies::eliminate_possibilities_using_y_wing::EliminatePossibilitiesUsingYWing;
use crate::solving::strategies::set_hidden_singles::SetHiddenSingles;
use crate::solving::strategies::set_single_randomly::SetSingleRandomly;
use crate::solving::traits::SudokuSolvingStrategy;
use crate::traits::Difficulty;
use crate::traits::Sudoku;
use crate::traits::SudokuTemplate;
use itertools::Itertools;

pub fn solve(sudoku: &Sudoku) -> Sudoku {
    let strategies = implemented_strategies();

    let mut template = SudokuTemplate::from(sudoku.clone());
    while strategies.iter().any(|s| s.solve(&mut template)) {}

    Sudoku::from(template)
}

pub fn solve_with_difficulty(sudoku: &Sudoku, difficulty: Difficulty) -> Sudoku {
    let strategies = implemented_strategies()
        .into_iter()
        .filter(|strategy| strategy.difficulty() <= difficulty)
        .collect_vec();

    let mut template = SudokuTemplate::from(sudoku.clone());
    while strategies.iter().any(|s| s.solve(&mut template)) {}

    Sudoku::from(template)
}

pub fn solve_with_guessing(sudoku: &Sudoku) -> Sudoku {
    let mut strategies = implemented_strategies();
    strategies.push(Box::new(SetSingleRandomly {}));

    let mut template = SudokuTemplate::from(sudoku.clone());
    while strategies.iter().any(|s| s.solve(&mut template)) {}

    Sudoku::from(template)
}

fn implemented_strategies() -> Vec<Box<dyn SudokuSolvingStrategy>> {
    vec![
        Box::new(EliminatePossibilitiesUsingExistingSingles {}),
        Box::new(SetHiddenSingles {}),
        Box::new(EliminatePossibilitiesUsingPointing {}),
        Box::new(EliminatePossibilitiesUsingNakedPairs {}),
        Box::new(EliminatePossibilitiesUsingHiddenGroups {}),
        Box::new(EliminatePossibilitiesUsingXWing {}),
        Box::new(EliminatePossibilitiesUsingYWing {}),
    ]
}
