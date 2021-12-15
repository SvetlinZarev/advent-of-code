use crate::astar_v1::a_star_v1;

pub mod astar_pf;
pub mod astar_v1;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct Position {
    r: usize,
    c: usize,
}
impl Position {
    pub fn new(r: usize, c: usize) -> Self {
        Position { r, c }
    }

    pub fn manhattan(&self, other: Position) -> u32 {
        let dx = self.r.max(other.r) - self.r.min(other.r);
        let dy = self.c.max(other.c) - self.c.min(other.c);

        (dx + dy) as u32
    }

    pub fn left(&self) -> Option<Position> {
        if self.c == 0 {
            return None;
        }

        Some(Position::new(self.r, self.c - 1))
    }

    pub fn right(&self, limit: Position) -> Option<Position> {
        if self.c >= limit.c {
            return None;
        }

        Some(Position::new(self.r, self.c + 1))
    }

    pub fn up(&self) -> Option<Position> {
        if self.r == 0 {
            return None;
        }

        Some(Position::new(self.r - 1, self.c))
    }

    pub fn down(&self, limit: Position) -> Option<Position> {
        if self.r >= limit.r {
            return None;
        }

        Some(Position::new(self.r + 1, self.c))
    }
}

pub fn part_one(grid: &[Vec<u8>]) -> u32 {
    a_star_v1(
        grid,
        Position::new(0, 0),
        Position::new(grid.len() - 1, grid[0].len() - 1),
        |_, _| 0,
    )
}

pub fn part_two(grid: &[Vec<u8>]) -> u32 {
    a_star_v1(
        grid,
        Position::new(0, 0),
        Position::new(grid.len() - 1, grid[0].len() - 1),
        |_, _| 0,
    )
}

pub fn expand_grid(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut expanded = vec![vec![0; grid[0].len() * 5]; grid.len() * 5];
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            expanded[r][c] = grid[r][c];
        }
    }

    let grid_cols = grid[0].len();
    for r in 0..grid.len() {
        for c in grid_cols..expanded[r].len() {
            expanded[r][c] = ((expanded[r][c - grid_cols] + 1) % 10).max(1);
        }
    }

    let grid_rows = grid.len();
    for r in grid_rows..expanded.len() {
        for c in 0..expanded[r].len() {
            expanded[r][c] = ((expanded[r - grid_rows][c] + 1) % 10).max(1);
        }
    }

    expanded
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_numeric_grid;

    #[test]
    fn test_part_one() {
        let input = parse_numeric_grid(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&input);

        assert_eq!(656, answer);
    }

    #[test]
    fn test_part_two() {
        let input = parse_numeric_grid(load_text_input_from_file("inputs/input.txt"));
        let input = expand_grid(&input);
        let answer = part_two(&input);

        assert_eq!(2979, answer);
    }
}
