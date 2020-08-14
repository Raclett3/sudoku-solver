mod board;
mod solver;

use board::Board;
use solver::solve;
use std::cmp::Ordering::*;
use std::io::stdin;
use std::process::exit;

fn read_line() -> Option<String> {
    let mut s = String::new();
    let result = stdin().read_line(&mut s);
    if result.is_ok() && result.unwrap() > 1 {
        Some(s)
    } else {
        None
    }
}

fn read_numbers() -> Option<Vec<usize>> {
    read_line().map(|line| {
        line.trim_end()
            .split(' ')
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse().unwrap_or(0))
            .collect()
    })
}

fn parse_input_into_board() -> Result<Board, &'static str> {
    let parsed: Vec<Vec<_>> = std::iter::repeat(())
        .map(|_| read_numbers())
        .take_while(|x| x.is_some())
        .flatten()
        .collect();

    let side_length = parsed.len();

    let mut num = 1;
    let size = loop {
        match (num * num).cmp(&side_length) {
            Less => (),
            Equal => break num,
            Greater => {
                return Err("Invalid input format: Number of columns should be a square number")
            }
        }
        num += 1;
    };

    let mut board = Board::new(size);

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

    let board = match parse_input_into_board() {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    if let Some(solved) = solve(&board) {
        println!("{}", solved);
    } else {
        eprintln!("No valid solution found");
        exit(1);
    }
}
