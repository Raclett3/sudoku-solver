use std::io::stdin;

fn read_line() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s
}

fn read_numbers() -> Vec<usize> {
    read_line()
        .trim_end()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.parse().unwrap_or(0))
        .collect()
}

fn main() {
    println!("Sudoku Solver");

    let size = 3usize;
    let side_length = size * size;
    let board: Vec<Vec<_>> = (0..side_length).map(|_| read_numbers()).collect();

    if board.len() < side_length {
        eprintln!("Invalid input format: Not enough rows");
        std::process::exit(1);
    }

    for row in board.iter() {
        if row.len() < side_length {
            eprintln!("Invalid input format: Not enough columns");
            std::process::exit(1);
        }

        if row.len() > side_length {
            eprintln!("Invalid input format: Too many columns");
            std::process::exit(1);
        }

        if row.iter().any(|&x| x > side_length) {
            eprintln!("Invalid input format: Number out of range");
            std::process::exit(1);
        }
    }

    println!("{:?}", board);
}
