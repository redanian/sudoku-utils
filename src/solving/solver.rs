use itertools::{iproduct, Itertools};

use crate::printer::print;
use crate::solving::eliminate_possibilities_using_existing_singles::EliminatePossibilitiesUsingExistingSingles;
use crate::solving::eliminate_possibilities_using_hidden_groups::EliminatePossibilitiesUsingHiddenCombinationsGroups;
use crate::solving::eliminate_possibilities_using_naked_pairs::EliminatePossibilitiesUsingNakedPairs;
use crate::solving::eliminate_possibilities_using_pointing::EliminatePossibilitiesUsingPointing;
use crate::solving::set_naked_singles::SetNakedSingles;
use crate::solving::traits::SudokuTemplateTransformer;
use crate::traits::Sudoku;
use crate::traits::SudokuTemplate;

pub fn solve(sudoku: &Sudoku) -> Sudoku {
    let mut template = SudokuTemplate::from(sudoku.clone());

    let transformers: Vec<Box<dyn SudokuTemplateTransformer>> = vec![
        Box::new(SetNakedSingles {}),
        Box::new(EliminatePossibilitiesUsingExistingSingles {}),
        Box::new(EliminatePossibilitiesUsingPointing {}),
        Box::new(EliminatePossibilitiesUsingNakedPairs {}),
        Box::new(EliminatePossibilitiesUsingHiddenCombinationsGroups {}),
    ];

    while transformers.iter().any(|t| t.transform(&mut template)) {}

    Sudoku::from(template)
}
