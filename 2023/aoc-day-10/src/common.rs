pub const START: u8 = b'S';

pub const DIRECTIONS: &[Direction] = &[
    Direction::Up,
    Direction::Left,
    Direction::Right,
    Direction::Down,
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn apply(self, r: usize, c: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => r.checked_sub(1).and_then(|r| Some((r, c))),
            Direction::Down => Some((r + 1, c)),
            Direction::Left => c.checked_sub(1).and_then(|c| Some((r, c))),
            Direction::Right => Some((r, c + 1)),
        }
    }

    pub fn normal(self) -> Self {
        self.rotr()
    }

    pub fn rotr(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn rotl(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

pub fn find_start(input: &[Vec<u8>]) -> (usize, usize) {
    for r in 0..input.len() {
        for c in 0..input[r].len() {
            if input[r][c] == START {
                return (r, c);
            }
        }
    }

    panic!("no starting point found")
}

pub fn initial_direction(grid: &[Vec<u8>], r: usize, c: usize) -> Direction {
    for &dir in DIRECTIONS {
        let Some((nr, nc)) = dir.apply(r, c) else {
            continue;
        };

        if nr >= grid.len() || nc >= grid[nr].len() {
            continue;
        }

        match dir {
            Direction::Up => {
                if [b'|', b'7', b'F'].contains(&grid[nr][nc]) {
                    return dir;
                }
            }
            Direction::Down => {
                if [b'|', b'L', b'J'].contains(&grid[nr][nc]) {
                    return dir;
                }
            }
            Direction::Left => {
                if [b'-', b'L', b'F'].contains(&grid[nr][nc]) {
                    return dir;
                }
            }
            Direction::Right => {
                if [b'-', b'J', b'7'].contains(&grid[nr][nc]) {
                    return dir;
                }
            }
        }
    }

    panic!("cannot determine initial direction")
}
