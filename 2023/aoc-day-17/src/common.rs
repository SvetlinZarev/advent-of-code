#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
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

    pub fn apply_signed(self, r: isize, c: isize) -> (isize, isize) {
        match self {
            Direction::Up => (r - 1, c),
            Direction::Down => (r + 1, c),
            Direction::Left => (r, c - 1),
            Direction::Right => (r, c + 1),
        }
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

    pub fn vertical(self) -> bool {
        match self {
            Direction::Up => true,
            Direction::Down => true,
            Direction::Left => false,
            Direction::Right => false,
        }
    }
}



#[inline(always)]
pub fn step(
    input: &[u8],
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
    d: Direction,
    s: usize,
) -> Option<(usize, usize, u16)> {
    let (mut rx, mut cx) = (r as isize, c as isize);
    let mut cost = 0;

    for _ in 0..s {
        let (nr, nc) = d.apply_signed(rx, cx);
        if nr < 0 || nc < 0 || nr >= rows as isize || nc >= (cols - 1) as isize {
            return None;
        }

        (rx, cx) = (nr, nc);
        cost += (input[rx as usize * cols + cx as usize] - b'0') as u16;
    }

    Some((rx as usize, cx as usize, cost))
}
