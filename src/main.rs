mod board;
mod solver;

use board::Board;
use solver::solve;
use std::io::stdin;
use std::process::exit;

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

    for (y, row) in parsed.iter().enumerate() {
        if row.len() < side_length {
            return Err("Invalid input format: Not enough columns");
        }

        if row.len() > side_length {
            return Err("Invalid input format: Too many columns");
        }

        if row.iter().any(|&x| x > side_length) {
            return Err("Invalid input format: Number out of range");
        }

        for (x, &number) in row.iter().enumerate() {
            if number != 0 {
                if !board.allows_number(x, y, number) {
                    return Err("Invalid input: Some numbers conflict");
                }
                board.set_number(x, y, number);
            }
        }
    }

    Ok(board)
}

fn main() {
    println!("Sudoku Solver");

    let size = 3usize;
    let board = match parse_input_into_vec(size) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    if let Some(solved) = solve(&board) {
        println!("{}", solved);
    } else {
        eprintln!("No valid answer found");
        exit(1);
    }
}
