use crate::solving::eliminate_possibilities_using_existing_singles::EliminatePossibilitiesUsingExistingSingles;
use crate::solving::eliminate_possibilities_using_hidden_groups::EliminatePossibilitiesUsingHiddenGroups;
use crate::solving::eliminate_possibilities_using_naked_pairs::EliminatePossibilitiesUsingNakedPairs;
use crate::solving::eliminate_possibilities_using_pointing::EliminatePossibilitiesUsingPointing;
use crate::solving::eliminate_possibilities_using_x_wing::EliminatePossibilitiesUsingXWing;
use crate::solving::eliminate_possibilities_using_y_wing::EliminatePossibilitiesUsingYWing;
use crate::solving::set_hidden_singles::SetHiddenSingles;
use crate::solving::traits::SudokuSolvingStrategy;
use crate::traits::Sudoku;
use crate::traits::SudokuTemplate;

pub fn solve(sudoku: &Sudoku) -> Sudoku {
    let mut template = SudokuTemplate::from(sudoku.clone());

    let strategies: Vec<Box<dyn SudokuSolvingStrategy>> = vec![
        Box::new(SetHiddenSingles {}),
        Box::new(EliminatePossibilitiesUsingExistingSingles {}),
        Box::new(EliminatePossibilitiesUsingPointing {}),
        Box::new(EliminatePossibilitiesUsingNakedPairs {}),
        Box::new(EliminatePossibilitiesUsingHiddenGroups {}),
        Box::new(EliminatePossibilitiesUsingXWing {}),
        Box::new(EliminatePossibilitiesUsingYWing {}),
    ];

    while strategies.iter().any(|s| s.solve(&mut template)) {}

    Sudoku::from(template)
}
