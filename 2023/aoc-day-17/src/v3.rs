use aoc_shared::algo::BucketQueue;
use aoc_shared::grid::Direction;
use aoc_shared::util::BitSet;

use crate::common::step;

pub fn part_one(input: &[u8]) -> u16 {
    const MAX_STEPS: usize = 3;
    const SKIP_STEPS: usize = 0;

    dijkstra::<SKIP_STEPS, MAX_STEPS>(input, &[Direction::Right, Direction::Down])
}

pub fn part_two(input: &[u8]) -> u16 {
    const MAX_STEPS: usize = 10;
    const SKIP_STEPS: usize = 3;

    dijkstra::<SKIP_STEPS, MAX_STEPS>(input, &[Direction::Right, Direction::Down])
}

pub fn dijkstra<const SKIP: usize, const STEPS: usize>(
    grid: &[u8],
    initial_dir: &[Direction],
) -> u16 {
    let cols = grid.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = grid.len() / cols;

    let mut seen = BitSet::new(rows * cols * 2);
    let mut queue = BucketQueue::new((rows + cols) * 9);

    // Seed the queue with the starting elements
    for d in initial_dir.iter().copied() {
        let (mut r, mut c, mut loss) = step(grid, rows, cols, 0, 0, d, SKIP).unwrap();

        for _ in SKIP..STEPS {
            let Some((nr, nc, cost)) = step(grid, rows, cols, r, c, d, 1) else {
                break;
            };

            r = nr;
            c = nc;
            loss += cost;

            queue.push(loss as usize, (loss, r, c, d));
        }
    }

    while let Some((loss, r, c, d)) = queue.pop() {
        if (r, c) == (rows - 1, cols - 2) {
            return loss;
        }

        if !seen.mark(cache_idx(rows, cols, r, c, d)) {
            continue;
        }

        for d in [d.rotl(), d.rotr()] {
            let Some((mut r, mut c, mut cost)) = step(grid, rows, cols, r, c, d, SKIP) else {
                continue;
            };
            cost += loss;

            for _ in SKIP..STEPS {
                let Some((nr, nc, cst)) = step(grid, rows, cols, r, c, d, 1) else {
                    break;
                };

                r = nr;
                c = nc;
                cost += cst;

                if !seen.is_set(cache_idx(rows, cols, r, c, d)) {
                    queue.push(cost as usize, (cost, r, c, d));
                }
            }
        }
    }

    unreachable!()
}

#[inline(always)]
fn cache_idx(rows: usize, cols: usize, r: usize, c: usize, d: Direction) -> usize {
    d.vertical() as usize * rows * cols + r * cols + c
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(input.as_bytes());
        assert_eq!(638, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_two(input.as_bytes());
        assert_eq!(748, answer);
    }
}
