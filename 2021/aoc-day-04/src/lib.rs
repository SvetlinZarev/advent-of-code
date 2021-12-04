pub mod parsing;

pub use parsing::parse_input;

#[derive(Debug, Clone, Default)]
struct Board {
    board: [[u32; 5]; 5],
}

impl Board {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn modify(&mut self, r: usize, c: usize, val: u32) {
        self.board[r][c] = val;
    }

    pub fn index(&self, index: &mut [Vec<usize>], key: usize) {
        for r in 0..self.board.len() {
            for c in 0..self.board[r].len() {
                let pos = self.board[r][c] as usize;
                index[pos].push(key);
            }
        }
    }

    pub fn find(&self, value: u32) -> Option<(u32, u32)> {
        for r in 0..self.board.len() {
            for c in 0..self.board[r].len() {
                if self.board[r][c] == value {
                    return Some((r as u32, c as u32));
                }
            }
        }

        None
    }

    pub fn sum(&self) -> u32 {
        self.board.iter().flat_map(|r| r.iter()).sum()
    }
}

#[derive(Debug, Clone, Default)]
struct Stats {
    row: [u16; 5],
    col: [u16; 5],
    sum: u32,
    win: bool,
}

impl Stats {
    pub fn mark(&mut self, r: u32, c: u32, val: u32) -> bool {
        self.row[r as usize] += 1;
        self.col[c as usize] += 1;
        self.sum += val;

        self.win |= self.row[r as usize] == 5 || self.col[c as usize] == 5;
        self.win
    }

    pub fn sum(&self) -> u32 {
        self.sum
    }

    pub fn has_won(&self) -> bool {
        self.win
    }
}

pub fn part_one(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut index = vec![Vec::new(); numbers.len()];
    for (idx, board) in boards.iter().enumerate() {
        board.index(&mut index, idx);
    }

    let mut marks = vec![Stats::default(); boards.len()];

    for number in numbers.iter().copied() {
        for board_idx in index[number as usize].iter().copied() {
            let (r, c) = boards[board_idx].find(number).unwrap();

            if marks[board_idx].mark(r, c, number) {
                return (boards[board_idx].sum() - marks[board_idx].sum()) * number;
            }
        }
    }

    0
}

pub fn part_two(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut index = vec![Vec::new(); numbers.len()];
    for (idx, board) in boards.iter().enumerate() {
        board.index(&mut index, idx);
    }

    let mut last_to_win = None;
    let mut marks = vec![Stats::default(); boards.len()];

    for number in numbers.iter().copied() {
        for board_idx in index[number as usize].iter().copied() {
            let stats = &mut marks[board_idx];
            if stats.has_won() {
                continue;
            }

            let (r, c) = boards[board_idx].find(number).unwrap();
            if stats.mark(r, c, number) {
                last_to_win = Some((board_idx, number));
            }
        }
    }

    if let Some((board_idx, number)) = last_to_win {
        return (boards[board_idx].sum() - marks[board_idx].sum()) * number;
    }

    0
}
