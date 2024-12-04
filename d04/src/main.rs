struct WordSearch<'a> {
    board: Vec<&'a str>,
    w: usize,
    h: usize,
}

impl<'a> WordSearch<'a> {
    fn new(board: Vec<&'a str>) -> Self {
        let w = board[0].len();
        let h = board.len();
        Self { board, w, h }
    }

    fn right(&self, x: usize, y: usize, word: &str) -> bool {
        if x + word.len() > self.w {
            return false;
        }
        for (i, c) in word.chars().enumerate() {
            if self.board[y].chars().nth(x + i) != Some(c) {
                return false;
            }
        }
        true
    }

    fn left(&self, x: usize, y: usize, word: &str) -> bool {
        if x < word.len() - 1 {
            return false;
        }
        for (i, c) in word.chars().enumerate() {
            if self.board[y].chars().nth(x - i) != Some(c) {
                return false;
            }
        }
        true
    }

    fn down(&self, x: usize, y: usize, word: &str) -> bool {
        if y + word.len() > self.h {
            return false;
        }
        for (i, c) in word.chars().enumerate() {
            if self.board[y + i].chars().nth(x) != Some(c) {
                return false;
            }
        }
        true
    }

    fn up(&self, x: usize, y: usize, word: &str) -> bool {
        if y < word.len() - 1 {
            return false;
        }
        for (i, c) in word.chars().enumerate() {
            if self.board[y - i].chars().nth(x) != Some(c) {
                return false;
            }
        }
        true
    }

    fn down_right(&self, x: usize, y: usize, word: &str) -> bool {
        if x + word.len() > self.w || y + word.len() > self.h {
            return false;
        }
        for (i, c) in word.chars().enumerate() {
            if self.board[y + i].chars().nth(x + i) != Some(c) {
                return false;
            }
        }
        true
    }

    fn down_left(&self, x: usize, y: usize, word: &str) -> bool {
        if x < word.len() - 1 || y + word.len() > self.h {
            return false;
        }
        for (i, c) in word.chars().enumerate() {
            if self.board[y + i].chars().nth(x - i) != Some(c) {
                return false;
            }
        }
        true
    }

    fn up_right(&self, x: usize, y: usize, word: &str) -> bool {
        if y < word.len() - 1 || x + word.len() > self.w {
            return false;
        }
        for (i, c) in word.chars().enumerate() {
            if self.board[y - i].chars().nth(x + i) != Some(c) {
                return false;
            }
        }
        true
    }

    fn up_left(&self, x: usize, y: usize, word: &str) -> bool {
        if x < word.len() - 1 || y < word.len() - 1 {
            return false;
        }
        for (i, c) in word.chars().enumerate() {
            if self.board[y - i].chars().nth(x - i) != Some(c) {
                return false;
            }
        }
        true
    }

    fn count_matches(&self, word: &str) -> usize {
        let mut count = 0;
        let first_char = word.chars().next().unwrap();
        for (y, row) in self.board.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == first_char {
                    if self.right(x, y, word) {
                        count += 1;
                    }
                    if self.left(x, y, word) {
                        count += 1;
                    }
                    if self.down(x, y, word) {
                        count += 1;
                    }
                    if self.up(x, y, word) {
                        count += 1;
                    }
                    if self.down_right(x, y, word) {
                        count += 1;
                    }
                    if self.down_left(x, y, word) {
                        count += 1;
                    }
                    if self.up_right(x, y, word) {
                        count += 1;
                    }
                    if self.up_left(x, y, word) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn x_mas_check(&self, x: usize, y: usize) -> bool {
        if y + 2 > self.h {
            return false;
        }
        (self.down_right(x, y, "MAS") || self.down_right(x, y, "SAM"))
            && (self.up_right(x, y + 2, "MAS") || self.up_right(x, y + 2, "SAM"))
    }

    fn count_x_mas(&self) -> usize {
        let mut count = 0;
        for (y, row) in self.board.iter().enumerate() {
            for x in 0..row.len() {
                if self.x_mas_check(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let board = WordSearch::new(include_str!("input.txt").lines().collect::<Vec<_>>());
    let res1 = board.count_matches("XMAS");
    println!("{}", res1);

    let res2 = board.count_x_mas();
    println!("{}", res2);
}
