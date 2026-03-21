use clap::{Arg, ArgAction, Command};
use std::process::exit;
use sudoku_utils::{generate_sudoku_with_difficulty, Difficulty};
use sudoku_utils_cli::printing::print_as_grid;

const DEFAULT_DIFFICULTY: Difficulty = Difficulty::Hard;

fn main() {
    let matches = Command::new("Sudoku Generator")
        .about("Generates a sudoku puzzle")
        .arg(
            Arg::new("difficulty")
                .help("The difficulty of the puzzle (easy, medium, hard)")
                .default_value("hard")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("grid")
                .help("Print the sudoku as a 9x9 grid")
                .short('g')
                .long("grid")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let difficulty = parse_optional_difficulty(matches.get_one::<String>("difficulty"));
    let print_as_grid_flag = matches.get_flag("grid");

    let sudoku = generate_sudoku_with_difficulty(difficulty);

    if print_as_grid_flag {
        print_as_grid(&sudoku);
    } else {
        println!("{}", sudoku.to_string());
    }
}

fn parse_optional_difficulty(difficulty_opt: Option<&String>) -> Difficulty {
    difficulty_opt
        .map(
            |difficulty_string| match difficulty_string.to_ascii_lowercase().as_str() {
                "easy" => Difficulty::Easy,
                "medium" => Difficulty::Medium,
                "hard" => Difficulty::Hard,
                _ => {
                    eprintln!("[Error] Invalid difficulty: {difficulty_string}");
                    exit(1);
                }
            },
        )
        .unwrap_or(DEFAULT_DIFFICULTY)
}
