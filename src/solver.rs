use crate::board::Board;

fn exhaustive_search(board: &mut Board, x: usize, y: usize) -> bool {
    if y >= board.side_length {
        return true;
    }

    let next_x = (x + 1) % board.side_length;
    let next_y = if x == board.side_length - 1 {
        y + 1
    } else {
        y
    };

    if board.get_number(x, y).is_none() {
        for num in 1..=board.side_length {
            if !board.allows_number(x, y, num) {
                continue;
            }

            board.set_number(x, y, num);
        
            if exhaustive_search(board, next_x, next_y) {
                return true;
            }
            board.remove_number(x, y);
        }

        false
    } else {
        exhaustive_search(board, next_x, next_y)
    }
}

pub fn solve(board: &Board) -> Option<Board> {
    let mut board = board.clone();
    if exhaustive_search(&mut board, 0, 0) {
        Some(board)
    } else {
        None
    }
}