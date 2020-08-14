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
        let formatted = "1 2 3  4 5 6  7 8 9\nx x x  x x x  x x x\nx x x  x x x  x x x\n\nx x x  x x x  x x x\nx x x  x x x  x x x\nx x x  x x x  x x x\n\nx x x  x x x  x x x\nx x x  x x x  x x x\nx x x  x x x  x x x";
        assert_eq!(format!("{}", board), formatted);

        let mut board = Board::new(3);
        assert!(board.allows_number(0, 0, 1));
        assert!(!board.allows_number(0, 0, 0));
        assert!(!board.allows_number(0, 0, 10));
        board.set_number(8, 0, 1);
        board.set_number(0, 8, 1);
        board.set_number(1, 0, 2);
        board.set_number(0, 1, 3);
        board.set_number(8, 1, 4);
        board.set_number(1, 8, 5);
        assert!(!board.allows_number(0, 0, 1));
        assert!(!board.allows_number(0, 0, 2));
        assert!(!board.allows_number(0, 0, 3));
        assert!(board.allows_number(0, 0, 4));
        assert!(board.allows_number(0, 0, 5));
        assert!(!board.allows_number(1, 0, 6));
        assert!(board.allows_number(1, 1, 1));
        assert!(!board.allows_number(1, 1, 4));
    }
}
