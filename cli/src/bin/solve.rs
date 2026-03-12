use std::process::exit;

use clap::{Arg, Command};

use sudoku_utils::{solve, Sudoku};
use sudoku_utils_cli::printing::print_as_grid;

fn main() {
    let matches = Command::new("Sudoku solver")
        .about("Solves a sudoku")
        .arg(Arg::new("sudoku")
            .help("The sudoku puzzle to solve as 81 consecutive chars. Digits 1 to 9 are considered as entries, \
            everything else as empty cells.")
            .required(true)
            .index(1))
        .get_matches();

    let unsolved_sudoku = matches
        .get_one::<String>("sudoku")
        .unwrap()
        .parse::<Sudoku>()
        .unwrap_or_else(|e| {
            eprintln!("[Error] {e}");
            exit(1)
        });

    println!("Input: ");
    print_as_grid(&unsolved_sudoku);

    let solved_sudoku = solve(&unsolved_sudoku);
    println!("Output: ");
    print_as_grid(&solved_sudoku);
}

fn test() -> [[usize; 9]; 9] {
    [
        [0, 0, 6, 4, 1, 5, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 8, 0, 7, 0, 6, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 8, 1, 0],
        [0, 3, 1, 0, 7, 0, 2, 6, 0],
        [0, 6, 5, 0, 0, 0, 9, 0, 0],
        [0, 0, 0, 5, 0, 9, 0, 8, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 3],
        [0, 0, 0, 8, 4, 2, 6, 0, 0],
    ]
}
