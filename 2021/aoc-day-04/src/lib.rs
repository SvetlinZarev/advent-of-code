pub mod parsing;

pub use parsing::parse_input;

#[derive(Debug, Clone, Default)]
pub struct Board {
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

pub fn part_one(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut index = vec![Vec::new(); numbers.len()];
    for (idx, board) in boards.iter().enumerate() {
        board.index(&mut index, idx);
    }

    let mut marks = vec![([0; 5], [0; 5], 0u32); boards.len()];
    for number in numbers.iter().copied() {
        for board_idx in index[number as usize].iter().copied() {
            let (r, c) = boards[board_idx].find(number).unwrap();

            marks[board_idx].2 += number;
            marks[board_idx].1[c as usize] += 1;
            marks[board_idx].0[r as usize] += 1;

            if marks[board_idx].1[c as usize] == 5 || marks[board_idx].0[r as usize] == 5 {
                return (boards[board_idx].sum() - marks[board_idx].2) * number;
            }
        }
    }

    0
}
