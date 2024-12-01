use aoc_shared::hashing::{FxHashMap, FxHashSet};
use std::error::Error;

pub fn parse_input_fast(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    const LINE_LENGTH: usize = 14;

    let mut x = vec![];
    let mut y = vec![];

    x.reserve(input.len() / LINE_LENGTH);
    y.reserve(input.len() / LINE_LENGTH);

    for line in input.as_bytes().chunks_exact(LINE_LENGTH) {
        let a0 = (line[0] - b'0') as u32 * 10_000;
        let a1 = (line[1] - b'0') as u32 * 1_000;
        let a2 = (line[2] - b'0') as u32 * 100;
        let a3 = (line[3] - b'0') as u32 * 10;
        let a4 = (line[4] - b'0') as u32 * 1;
        let a = a0 + a1 + a2 + a3 + a4;

        let b0 = (line[8] - b'0') as u32 * 10_000;
        let b1 = (line[9] - b'0') as u32 * 1_000;
        let b2 = (line[10] - b'0') as u32 * 100;
        let b3 = (line[11] - b'0') as u32 * 10;
        let b4 = (line[12] - b'0') as u32 * 1;
        let b = b0 + b1 + b2 + b3 + b4;

        x.push(a);
        y.push(b);
    }

    Ok((x, y))
}

pub fn parse_input_generic(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
    let mut x = vec![];
    let mut y = vec![];

    for line in input.lines() {
        let Some((a, b)) = line.split_once("   ") else {
            return Err(format!("Invalid line: {}", line).into());
        };

        x.push(a.trim().parse()?);
        y.push(b.trim().parse()?);
    }

    Ok((x, y))
}

pub fn part_one(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    let mut a = a.to_vec();
    let mut b = b.to_vec();

    a.sort_unstable();
    b.sort_unstable();

    a.into_iter()
        .zip(b.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

pub fn part_two_v1(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    let mut seen = FxHashMap::default();
    seen.reserve(a.len() + 64);

    b.iter().for_each(|&x| *seen.entry(x).or_insert(0) += 1);
    a.iter().fold(0, |acc, &val| {
        acc + val * seen.get(&val).copied().unwrap_or(0)
    })
}

pub fn part_two_v2(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    let seen = a.iter().copied().collect::<FxHashSet<_>>();
    b.iter()
        .fold(0, |acc, &val| acc + val * seen.contains(&val) as u32)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_parsing() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (a, b) = parse_input_fast(&input).unwrap();
        let (p, q) = parse_input_generic(&input).unwrap();

        assert_eq!(a, p);
        assert_eq!(b, q);
    }

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (a, b) = parse_input_fast(&input).unwrap();

        let answer = part_one(&a, &b);
        assert_eq!(2375403, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (a, b) = parse_input_generic(&input).unwrap();

        let answer = part_two_v1(&a, &b);
        assert_eq!(23082277, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (a, b) = parse_input_generic(&input).unwrap();

        let answer = part_two_v2(&a, &b);
        assert_eq!(23082277, answer);
    }
}
