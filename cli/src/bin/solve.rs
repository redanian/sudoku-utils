use std::process::exit;

use clap::{Arg, Command};

use sudoku_utils::{solve, Sudoku};

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
    print_sudoku(&unsolved_sudoku);

    let solved_sudoku = solve(&unsolved_sudoku);
    println!("Output: ");
    print_sudoku(&solved_sudoku);
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

fn print_sudoku(sudoku: &Sudoku) {
    println!(" {}", "-".repeat(29));
    for (index, row) in sudoku.get_cells().iter().enumerate() {
        println!(
            "| {}  {}  {} | {}  {}  {} | {}  {}  {} |",
            non_zero_or_space(row[0]),
            non_zero_or_space(row[1]),
            non_zero_or_space(row[2]),
            non_zero_or_space(row[3]),
            non_zero_or_space(row[4]),
            non_zero_or_space(row[5]),
            non_zero_or_space(row[6]),
            non_zero_or_space(row[7]),
            non_zero_or_space(row[8])
        );
        if (index + 1) % 3 == 0 && index < 8 {
            println!("|{}|", "-".repeat(29));
        }
    }
    println!(" {}", "-".repeat(29));
}

fn non_zero_or_space(x: usize) -> String {
    if x != 0 { x.to_string() } else { String::from(" ") }
}
