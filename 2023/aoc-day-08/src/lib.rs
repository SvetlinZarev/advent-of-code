use std::error::Error;
use std::str::FromStr;

use aoc_shared::hashing::FxHashMap;
use num::Integer;

type HashMap<K, V> = FxHashMap<K, V>;

pub fn run(input: &str) -> i64 {
    let input = parse_input(input);
    part_two(&input) as i64
}

#[derive(Debug)]
pub struct Input<'a> {
    lr: Vec<Dir>,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Dir {
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => return Err(format!("Invalid direction: {}", s).into()),
        })
    }
}

pub fn parse_input(input: &str) -> Input {
    let (lr, net) = input.split_once('\n').unwrap();
    let lr = lr
        .split_inclusive("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    let mut map = HashMap::default();
    map.reserve(1024);

    net.trim()
        .lines()
        .map(|l| (&l[0..3], &l[7..10], &l[12..15]))
        .for_each(|(k, l, r)| {
            map.insert(k, (l, r));
        });

    Input { lr, map }
}

pub fn part_one(input: &Input) -> u64 {
    solve(input, "AAA", |node| node == "ZZZ")
}

pub fn part_two(input: &Input) -> u64 {
    // All of the starting nodes (i.e. ones ending with 'A') appear only
    // once in the map. Each starting node leads to a different node
    // ending with 'Z' (i.e., there is a 1-1 mapping between a starting
    // and ending node). If we treat the ending nodes as a starting nodes,
    // then they form a cyclic path - each ending node leads back to itself.
    // without passing through any other nodes ending with `Z`. The length
    // of that cycle appears to be the same length as the path from an A node
    // to the corresponding Z node
    input
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .map(|key| solve(input, key, |node| node.ends_with('Z')))
        .fold(1, |acc, val| acc.lcm(&val))
}

fn solve(input: &Input, start: &str, finish: impl Fn(&str) -> bool) -> u64 {
    let mut current = start;

    for (steps, dir) in input.lr.iter().copied().cycle().enumerate() {
        let (left, right) = input.map.get(current).copied().unwrap();
        current = match dir {
            Dir::Left => left,
            Dir::Right => right,
        };

        if finish(current) {
            return (steps + 1) as u64;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input);

        let answer = part_one(&input);
        assert_eq!(15_871, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input);

        let answer = part_two(&input);
        assert_eq!(11_283_670_395_017, answer);
    }
}
