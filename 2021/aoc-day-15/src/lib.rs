use aoc_shared::hashing::{FnvHasher, HashBuilder};
use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

type BestScores = HashMap<Position, u32, HashBuilder<FnvHasher>>;

pub fn part_one(grid: &[Vec<u8>]) -> u32 {
    a_star(
        grid,
        Position::new(0, 0),
        Position::new(grid.len() - 1, grid[0].len() - 1),
        |_, _| 0,
    )
}

pub fn part_two(grid: &[Vec<u8>]) -> u32 {
    a_star(
        grid,
        Position::new(0, 0),
        Position::new(grid.len() - 1, grid[0].len() - 1),
        |_, _| 0,
    )
}

pub fn a_star_pf(grid: &[Vec<u8>]) -> u32 {
    pathfinding::directed::astar::astar(
        &(0, 0),
        |&(r, c)| {
            let mut s = Vec::with_capacity(4);
            if r > 0 {
                s.push(((r - 1, c), grid[r - 1][c] as usize));
            }
            if c > 0 {
                s.push(((r, c - 1), grid[r][c - 1] as usize));
            }
            if c < grid[r].len() - 1 {
                s.push(((r, c + 1), grid[r][c + 1] as usize));
            }
            if r < grid.len() - 1 {
                s.push(((r + 1, c), grid[r + 1][c] as usize));
            }

            s
        },
        |&(r, c)| {
            Position::new(r, c).manhattan(Position::new(grid.len() - 1, grid[0].len() - 1)) as usize
        },
        |&(r, c)| r == grid.len() - 1 && c == grid[0].len() - 1,
    )
    .unwrap()
    .1 as u32
}

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

pub fn a_star<H: Copy + Fn(Position, Position) -> u32>(
    grid: &[Vec<u8>],
    start: Position,
    dst: Position,
    heuristic: H,
) -> u32 {
    let mut best = HashMap::with_hasher(HashBuilder::<FnvHasher>::default());
    let mut open = BinaryHeap::new();

    // (Reverse(f_score), Reverse(g_score), (row, col))
    open.push((Reverse(0), Reverse(0), start));

    while let Some((_, Reverse(g), pos)) = open.pop() {
        if pos == dst {
            return best.get(&pos).copied().unwrap();
        }

        if let Some(pos) = pos.up() {
            on_neighbour(heuristic, pos, dst, g, grid, &mut best, &mut open);
        }
        if let Some(pos) = pos.left() {
            on_neighbour(heuristic, pos, dst, g, grid, &mut best, &mut open);
        }
        if let Some(pos) = pos.right(dst) {
            on_neighbour(heuristic, pos, dst, g, grid, &mut best, &mut open);
        }
        if let Some(pos) = pos.down(dst) {
            on_neighbour(heuristic, pos, dst, g, grid, &mut best, &mut open);
        }
    }

    panic!("Cannot find path in the current grid")
}

#[inline(always)]
fn on_neighbour<H: Fn(Position, Position) -> u32>(
    heuristic: H,
    pos: Position,
    dst: Position,
    current_g: u32,
    grid: &[Vec<u8>],
    best: &mut BestScores,
    pq: &mut BinaryHeap<(Reverse<u32>, Reverse<u32>, Position)>,
) {
    let g_score = current_g + grid[pos.r][pos.c] as u32;

    match best.entry(pos) {
        Entry::Vacant(e) => {
            e.insert(g_score);
        }

        Entry::Occupied(mut e) => {
            if *e.get() <= g_score {
                return;
            }

            e.insert(g_score);
        }
    }

    let f_score = heuristic(pos, dst) + g_score;
    pq.push((Reverse(f_score), Reverse(g_score), pos));
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
