#[derive(std::fmt::Debug, Clone)]
pub struct Board {
    size: usize,
    side_length: usize,
    board: Vec<Vec<Option<usize>>>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.side_length {
            for x in 0..self.side_length {
                let delimiter = if x == self.side_length - 1 {
                    '\n'
                } else {
                    ' '
                };
                if let Some(number) = self.get_number(x, y) {
                    write!(f, "{}{}", number, delimiter)?;
                } else {
                    write!(f, "x{}", delimiter)?;
                }
            }
        }
        Ok(())
    }
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
            if 1 <= number && number <= self.side_length {
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

    pub fn allows_number(&self, x: usize, y: usize, number: usize) -> bool {
        if number < 1 || self.side_length < number {
            return false;
        }

        if self.get_number(x, y).is_some() {
            return false;
        }

        for x in 0..self.side_length {
            if self.get_number(x, y) == Some(number) {
                return false;
            }
        }

        for y in 0..self.side_length {
            if self.get_number(x, y) == Some(number) {
                return false;
            }
        }

        let origin_x = x - x % self.size;
        let origin_y = y - y % self.size;

        for x in 0..self.size {
            for y in 0..self.size {
                if self.get_number(origin_x + x, origin_y + y) == Some(number) {
                    return false;
                }
            }
        }

        true
    }
}
