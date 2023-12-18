use aoc_shared::grid::Point;

use crate::common::BucketQueue;

pub fn part_one(input: &[u8]) -> u16 {
    const MAX_STEPS: usize = 3;
    const SKIP_STEPS: usize = 0;

    dijkstra::<SKIP_STEPS, MAX_STEPS>(input, &[Point::RIGHT, Point::DOWN])
}

pub fn part_two(input: &[u8]) -> u16 {
    const MAX_STEPS: usize = 10;
    const SKIP_STEPS: usize = 3;

    dijkstra::<SKIP_STEPS, MAX_STEPS>(input, &[Point::RIGHT, Point::DOWN])
}

pub fn dijkstra<const SKIP: usize, const STEPS: usize>(grid: &[u8], initial_dir: &[Point]) -> u16 {
    let cols = grid.iter().position(|&x| x == b'\n').unwrap() + 1;
    let rows = grid.len() / cols;
    let target = Point::new(cols as i64 - 2, rows as i64 - 1);

    let mut queue = BucketQueue::new((rows + cols) * 9);
    let mut seen = vec![u16::MAX; grid.len() * 2];

    // Seed the queue with the starting elements
    for d in initial_dir.iter().copied() {
        let (mut p, mut loss) = step(grid, rows, cols, Point::ZERO, d, SKIP).unwrap();

        for _ in SKIP..STEPS {
            let Some((q, cst)) = step(grid, rows, cols, p, d, 1) else {
                break;
            };

            p = q;
            loss += cst;

            seen[(d.y != 0) as usize * grid.len() + p.r() * cols + p.c()] = loss;
            queue.push(loss as usize, (loss, p, d));
        }
    }

    while let Some((loss, p, d)) = queue.pop() {
        if p == target {
            return loss;
        }

        for d in [d.rotl(), d.rotr()] {
            let Some((mut src, mut cost)) = step(grid, rows, cols, p, d, SKIP) else {
                continue;
            };
            cost += loss;

            for _ in SKIP..STEPS {
                let Some((dst, cst)) = step(grid, rows, cols, src, d, 1) else {
                    break;
                };

                src = dst;
                cost += cst;

                if cost < seen[(d.y != 0) as usize * grid.len() + src.r() * cols + src.c()] {
                    seen[(d.y != 0) as usize * grid.len() + src.r() * cols + src.c()] = cost;
                    queue.push(cost as usize, (cost, src, d));
                }
            }
        }
    }

    unreachable!()
}

#[inline(always)]
pub fn step(
    input: &[u8],
    rows: usize,
    cols: usize,
    p: Point,
    d: Point,
    s: usize,
) -> Option<(Point, u16)> {
    let mut cost = 0;
    let mut x = p;

    for _ in 0..s {
        x += d;
        if !x.not_negative_coordinates() || x.c() >= cols - 1 || x.r() >= rows {
            return None;
        }

        cost += (input[x.r() * cols + x.c()] - b'0') as u16;
    }

    Some((x, cost))
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
