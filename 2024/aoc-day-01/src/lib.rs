use aoc_shared::hashing::{FxHashMap, FxHashSet};
use std::error::Error;

pub fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>), Box<dyn Error>> {
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
    let mut seen = FxHashSet::default();
    seen.reserve(a.len() + 64);

    a.iter().for_each(|&x| {
        seen.insert(x);
    });

    b.iter()
        .fold(0, |acc, &val| acc + val * seen.contains(&val) as u32)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (a, b) = parse_input(input.trim()).unwrap();

        let answer = part_one(&a, &b);
        assert_eq!(2375403, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (a, b) = parse_input(input.trim()).unwrap();

        let answer = part_two_v1(&a, &b);
        assert_eq!(23082277, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (a, b) = parse_input(input.trim()).unwrap();

        let answer = part_two_v2(&a, &b);
        assert_eq!(23082277, answer);
    }
}
