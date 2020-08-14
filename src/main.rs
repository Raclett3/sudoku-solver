mod board;

use board::Board;
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

fn parse_input_into_vec(size: usize) -> Result<Board, &'static str> {
    let side_length = size * size;
    let mut board = Board::new(size);
    let parsed: Vec<Vec<_>> = (0..side_length).map(|_| read_numbers()).collect();

    if parsed.len() < side_length {
        return Err("Invalid input format: Not enough rows");
    }

    for (x, row) in parsed.iter().enumerate() {
        if row.len() < side_length {
            return Err("Invalid input format: Not enough columns");
        }

        if row.len() > side_length {
            return Err("Invalid input format: Too many columns");
        }

        if row.iter().any(|&x| x > side_length) {
            return Err("Invalid input format: Number out of range");
        }

        for (y, &number) in row.iter().enumerate() {
            board.set_number(x, y, number);
        }
    }

    Ok(board)
}

fn main() {
    println!("Sudoku Solver");

    let size = 3usize;
    let board = parse_input_into_vec(size);

    match &board {
        Ok(parsed) => println!("{:?}", parsed),
        Err(err) => eprintln!("{}", err),
    };
}
