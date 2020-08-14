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
    }
}
