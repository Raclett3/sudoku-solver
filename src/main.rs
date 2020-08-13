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

fn parse_input_into_vec(side_length: usize) -> Result<Vec<Vec<usize>>, &'static str> {
    let board: Vec<Vec<_>> = (0..side_length).map(|_| read_numbers()).collect();

    if board.len() < side_length {
        return Err("Invalid input format: Not enough rows");
    }

    for row in board.iter() {
        if row.len() < side_length {
            return Err("Invalid input format: Not enough columns");
        }

        if row.len() > side_length {
            return Err("Invalid input format: Too many columns");
        }

        if row.iter().any(|&x| x > side_length) {
            return Err("Invalid input format: Number out of range");
        }
    }

    Ok(board)
}

fn main() {
    println!("Sudoku Solver");

    let size = 3usize;
    let side_length = size * size;
    let board = parse_input_into_vec(side_length);

    match &board {
        Ok(parsed) => println!("{:?}", parsed),
        Err(err) => eprintln!("{}", err),
    };
}
