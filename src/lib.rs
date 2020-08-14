#![allow(dead_code)]

mod board;

#[cfg(test)]
mod test {
    use super::board::Board;

    #[test]
    fn test_board() {
        let mut board = Board::new(3);
        board.set_number(1, 1, 2);
        board.set_number(2, 2, 0);
        board.set_number(3, 3, 10);
        assert_eq!(board.get_number(0, 0), None);
        assert_eq!(board.get_number(9, 9), None);
        assert_eq!(board.get_number(1, 1), Some(2));
        assert_eq!(board.get_number(2, 2), None);
        assert_eq!(board.get_number(3, 3), None);
        board.remove_number(1, 1);
        assert_eq!(board.get_number(1, 1), None);

        let mut board = Board::new(3);
        for x in 0..9 {
            board.set_number(x, 0, x + 1);
        }
        let formatted = "1 2 3 4 5 6 7 8 9\nx x x x x x x x x\nx x x x x x x x x\nx x x x x x x x x\nx x x x x x x x x\nx x x x x x x x x\nx x x x x x x x x\nx x x x x x x x x\nx x x x x x x x x\n";
        assert_eq!(format!("{}", board), formatted);
    }
}
