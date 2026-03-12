use crate::traits::SudokuTemplate;

pub(crate) fn print(sudoku: &SudokuTemplate) {
    println!("[DEBUG] current template state: ");
    println!(" {} ", "-".repeat(111));
    for x in 0..9 {
        print!("|| ");
        for y in 0..9 {
            let possibilities = sudoku.cells[x][y].possible_values();
            for n in 1..=9 {
                if possibilities.contains(&n) {
                    print!("{}", n);
                } else {
                    print!(" ");
                }
            }
            print!(" |");
            if (y + 1) % 3 == 0 {
                print!("|");
            }
            print!(" ");
        }
        println!();
        if (x + 1) % 3 == 0 {
            println!(" {} ", "-".repeat(111));
        } else {
            println!("|{}|", "|           |           |           |".repeat(3));
        }
    }
}
