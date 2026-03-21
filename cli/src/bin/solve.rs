use std::process::exit;

use clap::{Arg, ArgAction, Command};

use sudoku_utils::{solve, Sudoku};
use sudoku_utils_cli::printing::print_as_grids;

fn main() {
    let matches = Command::new("Sudoku Solver")
        .about("Solves a sudoku")
        .arg(Arg::new("sudoku")
            .help("The sudoku puzzle to solve as 81 consecutive chars. Digits 1 to 9 are considered as entries, \
            everything else as empty cells.")
            .required(true)
            .index(1))
        .arg(
            Arg::new("grid")
                .help("Print the input and solution as 9x9 grids side by side")
                .short('g')
                .long("grid")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let unsolved_sudoku = matches
        .get_one::<String>("sudoku")
        .unwrap()
        .parse::<Sudoku>()
        .unwrap_or_else(|e| {
            eprintln!("[Error] {e}");
            exit(1)
        });
    let print_as_grid_flag = matches.get_flag("grid");

    let solved_sudoku = solve(&unsolved_sudoku);

    if print_as_grid_flag {
        print_as_grids(&unsolved_sudoku, &solved_sudoku);
    } else {
        println!("{}", solved_sudoku.to_string());
    }
}
