use aoc_shared::hashing::FnvHasher;
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::hash::BuildHasherDefault;

const DIR: &[(isize, isize)] = &[(0, -1), (-1, 0), (1, 0), (0, 1)];

const NOT_PROCESSED: i32 = -1;
const MULTI_ID: i32 = -2;

const TOTAL_DIST_LIMIT: usize = 10_000;

pub fn parse_input(input: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut result = vec![];

    for line in input.lines() {
        let Some((x, y)) = line.split_once(", ") else {
            return Err(format!("invalid input: {}", line).into());
        };

        let x = x.parse()?;
        let y = y.parse()?;
        result.push((x, y));
    }

    Ok(result)
}

pub fn part_one(input: &Vec<(usize, usize)>) -> usize {
    let (min_x, min_y, max_x, max_y) = boundaries(input);
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut infinite = vec![false; input.len()];
    let mut area = vec![0usize; input.len()];

    let mut grid = vec![NOT_PROCESSED; width * height];
    let mut queue = VecDeque::new();

    for (idx, &(x, y)) in input.iter().enumerate() {
        queue.push_back((idx as i32, x - min_x, y - min_y));
    }

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (id, x, y) = queue.pop_front().unwrap();
            let idx = y * width + x;

            if grid[idx] == NOT_PROCESSED {
                grid[idx] = id;
                area[id as usize] += 1;

                for &(dx, dy) in DIR {
                    let Some(nx) = x.checked_add_signed(dx) else {
                        infinite[id as usize] = true;
                        continue;
                    };

                    let Some(ny) = y.checked_add_signed(dy) else {
                        infinite[id as usize] = true;
                        continue;
                    };

                    if nx >= width || ny >= height {
                        infinite[id as usize] = true;
                        continue;
                    }

                    queue.push_back((id, nx, ny))
                }
            } else if grid[idx] >= 0 && grid[idx] != id {
                let new_id = id as usize;
                let old_id = grid[idx] as usize;

                let d1 = manhattan(x, y, input[new_id].0 - min_x, input[new_id].1 - min_y);
                let d2 = manhattan(x, y, input[old_id].0 - min_x, input[old_id].1 - min_y);

                if d1 == d2 {
                    grid[idx] = MULTI_ID;
                    area[old_id] -= 1;
                }
            }
        }
    }

    area.iter()
        .copied()
        .zip(infinite.iter().copied())
        .filter(|&(_, inf)| !inf)
        .map(|(area, _)| area)
        .max()
        .unwrap_or(0)
}

pub fn part_two(input: &Vec<(usize, usize)>) -> usize {
    let mut avg_x = 0;
    let mut avg_y = 0;
    for &(x, y) in input {
        avg_x += x;
        avg_y += y;
    }

    let avg_x = (avg_x / input.len()) as isize;
    let avg_y = (avg_y / input.len()) as isize;

    let mut area = HashSet::with_hasher(BuildHasherDefault::<FnvHasher>::default());
    let mut queue = VecDeque::new();
    queue.push_back((avg_x, avg_y));

    while let Some((x, y)) = queue.pop_front() {
        let total_dist: usize = input
            .iter()
            .map(|&(c, r)| s_manhattan(x, y, c as isize, r as isize))
            .sum();

        if total_dist >= TOTAL_DIST_LIMIT {
            continue;
        }

        if !area.insert((x, y)) {
            continue;
        }

        for &(dx, dy) in DIR {
            let nx = x + dx;
            let ny = y + dy;

            if !area.contains(&(nx, ny)) {
                queue.push_back((nx, ny));
            }
        }
    }

    area.len()
}

fn boundaries(input: &[(usize, usize)]) -> (usize, usize, usize, usize) {
    let mut min_x = input[0].0;
    let mut min_y = input[0].1;
    let mut max_x = input[0].0;
    let mut max_y = input[0].1;

    for &(x, y) in input {
        min_x = min_x.min(x);
        max_x = max_x.max(x);

        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    (min_x, min_y, max_x, max_y)
}

fn manhattan(x: usize, y: usize, c: usize, r: usize) -> usize {
    x.abs_diff(c) + y.abs_diff(r)
}

fn s_manhattan(x: isize, y: isize, c: isize, r: isize) -> usize {
    x.abs_diff(c) + y.abs_diff(r)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_one(&input);
        assert_eq!(4829, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input).unwrap();

        let answer = part_two(&input);
        assert_eq!(46966, answer);
    }
}
