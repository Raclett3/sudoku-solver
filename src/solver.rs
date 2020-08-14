use crate::board::Board;

fn exhaustive_search(board: &mut Board, x: usize, y: usize, count_solutions: bool) -> usize {
    if y >= board.side_length {
        return 1;
    }

    let next_x = (x + 1) % board.side_length;
    let next_y = if x == board.side_length - 1 { y + 1 } else { y };

    if board.get_number(x, y).is_none() {
        let mut count = 0;
        for num in 1..=board.side_length {
            if !board.allows_number(x, y, num) {
                continue;
            }

            board.set_number(x, y, num);

            count += exhaustive_search(board, next_x, next_y, count_solutions);

            if count > 0 && !count_solutions {
                return 1;
            }

            board.remove_number(x, y);
        }

        count
    } else {
        exhaustive_search(board, next_x, next_y, count_solutions)
    }
}

pub fn solve(board: &Board, count_solutions: bool) -> Option<(Board, usize)> {
    let mut board = board.clone();
    let count = exhaustive_search(&mut board, 0, 0, count_solutions);
    if count > 0 {
        Some((board, count))
    } else {
        None
    }
}
