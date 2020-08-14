#[derive(std::fmt::Debug)]
pub struct Board {
    size: usize,
    side_length: usize,
    board: Vec<Vec<Option<usize>>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let side_length = size * size;
        Self {
            size,
            side_length,
            board: vec![vec![None; side_length]; side_length],
        }
    }

    pub fn get_number(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.side_length && y < self.side_length {
            self.board[y][x]
        } else {
            None
        }
    }

    pub fn set_number(&mut self, x: usize, y: usize, number: usize) {
        if x < self.side_length && y < self.side_length {
            if 0 < number && number <= self.side_length {
                self.board[y][x] = Some(number);
            } else {
                self.board[y][x] = None;
            }
        }
    }

    pub fn remove_number(&mut self, x: usize, y: usize) {
        if x < self.side_length && y < self.side_length {
            self.board[y][x] = None;
        }
    }
}
