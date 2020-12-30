use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

pub const DAY: usize = 13;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (dp, (knights, matrix)) = measure(DAY, "parsing", || parse_input(&input));
    let (d1, _) = measure(DAY, "part 1", || solve(knights, &matrix));

    let matrix = to_part_2_input(knights, &matrix);
    let (d2, _) = measure(DAY, "part 2", || solve(knights + 1, &matrix));

    dp + d1 + d2
}

pub fn parse_input(input: &str) -> (usize, Vec<i64>) {
    let mut knights = HashMap::new();
    let mut entries = vec![];

    for line in input.lines() {
        let mut idx = line.find(' ').unwrap();
        let target = &line[..idx];

        idx = line.rfind(' ').unwrap();
        let knight = &line[idx + 1..line.len() - 1];

        let mut next_id = knights.len();
        let id_target = *knights.entry(target).or_insert(next_id);

        next_id = knights.len();
        let id_knight = *knights.entry(knight).or_insert(next_id);

        let diff = if let Some(idx) = line.find("gain ") {
            let end = line[idx + 5..].find(' ').unwrap();
            line[idx + 5..idx + 5 + end].parse::<i64>().unwrap()
        } else if let Some(idx) = line.find("lose ") {
            let end = line[idx + 5..].find(' ').unwrap();
            -line[idx + 5..idx + 5 + end].parse::<i64>().unwrap()
        } else {
            panic!("Unexpected input: {}", line);
        };

        entries.push((id_target, id_knight, diff));
    }

    let mut matrix = vec![0i64; knights.len().pow(2)];
    for (id_a, id_b, diff) in entries {
        let idx = id_a * knights.len() + id_b;
        matrix[idx] = diff;
    }

    (knights.len(), matrix)
}

pub fn to_part_2_input(knights: usize, matrix: &[i64]) -> Vec<i64> {
    assert_eq!(knights.pow(2), matrix.len());

    let mut m = vec![0; (knights + 1).pow(2)];
    for i in 0..knights {
        for j in 0..knights {
            m[i * (knights + 1) + j] = matrix[i * knights + j];
        }
    }

    m
}

pub fn solve(knights: usize, matrix: &[i64]) -> i64 {
    let mut visited = vec![false; knights];
    visited[0] = true;

    compute(knights, matrix, &mut visited, knights - 1, 0)
}

fn compute(
    knights: usize,
    matrix: &[i64],
    visited: &mut [bool],
    remaining: usize,
    current: usize,
) -> i64 {
    if remaining == 0 {
        return matrix[current * knights + 0] + matrix[0 * knights + current];
    }

    let mut max_change = 0;
    // start from 1, because 0 is always fixed and does not change
    for k in 1..knights {
        if visited[k] {
            continue;
        }

        visited[k] = true;

        let diff = matrix[current * knights + k] + matrix[k * knights + current];
        max_change = max_change.max(diff + compute(knights, matrix, visited, remaining - 1, k));
        visited[k] = false;
    }

    max_change
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_parse_input() {
        let (knights, matrix) = parse_input(
            "\
            Alice would lose 4 happiness units by sitting next to George.\n\
            George would gain 45 happiness units by sitting next to Alice.\
            ",
        );

        assert_eq!(2, knights);
        assert_eq!(&[0, -4, 45, 0], matrix.as_slice());
    }

    #[test]
    fn test_to_part_2_input() {
        let (knights, matrix) = parse_input(
            "\
            Alice would lose 4 happiness units by sitting next to George.\n\
            George would gain 45 happiness units by sitting next to Alice.\
            ",
        );

        assert_eq!(2, knights);

        let matrix = to_part_2_input(knights, &matrix);
        assert_eq!((knights + 1).pow(2), matrix.len());
        assert_eq!(&[0, -4, 0, 45, 0, 0, 0, 0, 0], matrix.as_slice());
    }
    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let (knights, matrix) = parse_input(&input);

        let answer = solve(knights, &matrix);
        assert_eq!(664, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let (knights, matrix) = parse_input(&input);
        let matrix = to_part_2_input(knights, &matrix);
        let answer = solve(knights + 1, &matrix);
        assert_eq!(640, answer);
    }
}
