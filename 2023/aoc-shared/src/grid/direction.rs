#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    #[inline(always)]
    pub fn apply(self, r: usize, c: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => r.checked_sub(1).and_then(|r| Some((r, c))),
            Direction::Down => Some((r + 1, c)),
            Direction::Left => c.checked_sub(1).and_then(|c| Some((r, c))),
            Direction::Right => Some((r, c + 1)),
        }
    }

    #[inline(always)]
    pub const fn apply_signed(self, r: isize, c: isize) -> (isize, isize) {
        match self {
            Direction::Up => (r - 1, c),
            Direction::Down => (r + 1, c),
            Direction::Left => (r, c - 1),
            Direction::Right => (r, c + 1),
        }
    }

    #[inline(always)]
    pub const fn rotr(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    #[inline(always)]
    pub const fn rotl(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    #[inline(always)]
    pub const fn vertical(self) -> bool {
        match self {
            Direction::Up => true,
            Direction::Down => true,
            Direction::Left => false,
            Direction::Right => false,
        }
    }

    #[inline(always)]
    pub const fn horizontal(self) -> bool {
        !self.vertical()
    }
}
