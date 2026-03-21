use sudoku_utils::Sudoku;

pub fn print_as_grid(sudoku: &Sudoku) {
    let dashes = "-".repeat(29);
    let block_separator = format!("|{}|", dashes);
    println!(" {}", dashes);
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
            println!("{}", block_separator);
        }
    }
    println!(" {}", dashes);
}

pub fn print_as_grids(left: &Sudoku, right: &Sudoku) {
    let dashes = "-".repeat(29);
    let block_separator = format!("|{}|", dashes);

    println!(" {}     {}", dashes, dashes);

    for row in 0..9 {
        let left_row = left.get_cells()[row];
        let right_row = right.get_cells()[row];

        println!(
            "| {}  {}  {} | {}  {}  {} | {}  {}  {} |   | {}  {}  {} | {}  {}  {} | {}  {}  {} |",
            non_zero_or_space(left_row[0]),
            non_zero_or_space(left_row[1]),
            non_zero_or_space(left_row[2]),
            non_zero_or_space(left_row[3]),
            non_zero_or_space(left_row[4]),
            non_zero_or_space(left_row[5]),
            non_zero_or_space(left_row[6]),
            non_zero_or_space(left_row[7]),
            non_zero_or_space(left_row[8]),
            non_zero_or_space(right_row[0]),
            non_zero_or_space(right_row[1]),
            non_zero_or_space(right_row[2]),
            non_zero_or_space(right_row[3]),
            non_zero_or_space(right_row[4]),
            non_zero_or_space(right_row[5]),
            non_zero_or_space(right_row[6]),
            non_zero_or_space(right_row[7]),
            non_zero_or_space(right_row[8]),
        );

        if (row + 1) % 3 == 0 && row < 8 {
            println!("{}   {}", block_separator, block_separator);
        }
    }

    println!(" {}     {}", dashes, dashes);
}

fn non_zero_or_space(x: usize) -> String {
    if x != 0 {
        x.to_string()
    } else {
        String::from(" ")
    }
}
