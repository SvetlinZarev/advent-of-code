use crate::Position;
use aoc_shared::hashing::{FnvHasher, HashBuilder};
use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};

type BestScores = HashMap<Position, u32, HashBuilder<FnvHasher>>;

pub fn a_star_v1<H: Copy + Fn(Position, Position) -> u32>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_numeric_grid;

    #[test]
    fn test_a_start_v1() {
        let input = parse_numeric_grid(load_text_input_from_file("inputs/input.txt"));
        let answer = a_star_v1(
            &input,
            Position::new(0, 0),
            Position::new(input.len() - 1, input[0].len() - 1),
            |_, _| 0,
        );

        assert_eq!(656, answer);
    }
}
