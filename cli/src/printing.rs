use sudoku_utils::Sudoku;

pub fn print_as_grid(sudoku: &Sudoku) {
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
    if x != 0 {
        x.to_string()
    } else {
        String::from(" ")
    }
}
