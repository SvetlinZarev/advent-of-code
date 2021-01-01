use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

pub const DAY: usize = 16;

const KEY_CHILDREN: usize = 0;
const KEY_CATS: usize = 1;
const KEY_SAMOYEDS: usize = 2;
const KEY_POMERANIANS: usize = 3;
const KEY_AKITAS: usize = 4;
const KEY_VIZSLAS: usize = 5;
const KEY_GOLDFISH: usize = 6;
const KEY_TREES: usize = 7;
const KEY_CARS: usize = 8;
const KEY_PERFUMES: usize = 9;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (dp, parsed) = measure(DAY, "parsing", || parse_input(&input));
    let (d1, _) = measure(DAY, "part 1", || solve_part_one(&parsed));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(&parsed));

    dp + d1 + d2
}

fn parse_input(input: &str) -> Vec<[i8; 10]> {
    let mut aunts = Vec::with_capacity(500);
    aunts.resize_with(500, || [-1; 10]);

    for line in input.lines() {
        let mut line = &line[4..];
        let mut idx = line.find(':').unwrap();
        let id = line[..idx].parse::<usize>().unwrap() - 1;

        let aunt = &mut aunts[id];
        line = &line[idx + 2..];

        for thing in line.split(',') {
            let thing = thing.trim();
            idx = thing.find(':').unwrap();

            let attribute = &thing[..idx];
            let quantity = thing[idx + 2..].parse().unwrap();

            match attribute {
                "children" => aunt[KEY_CHILDREN] = quantity,
                "cars" => aunt[KEY_CARS] = quantity,
                "vizslas" => aunt[KEY_VIZSLAS] = quantity,
                "akitas" => aunt[KEY_AKITAS] = quantity,
                "goldfish" => aunt[KEY_GOLDFISH] = quantity,
                "pomeranians" => aunt[KEY_POMERANIANS] = quantity,
                "cats" => aunt[KEY_CATS] = quantity,
                "samoyeds" => aunt[KEY_SAMOYEDS] = quantity,
                "perfumes" => aunt[KEY_PERFUMES] = quantity,
                "trees" => aunt[KEY_TREES] = quantity,
                _ => panic!("Unknown attribute: {}", attribute),
            }
        }
    }

    aunts
}

fn solve_part_one(aunts: &[[i8; 10]]) -> usize {
    aunts
        .iter()
        .enumerate()
        .filter(|&(_, a)| a[KEY_CHILDREN] == 3 || a[KEY_CHILDREN] == -1)
        .filter(|&(_, a)| a[KEY_CATS] == 7 || a[KEY_CATS] == -1)
        .filter(|&(_, a)| a[KEY_SAMOYEDS] == 2 || a[KEY_SAMOYEDS] == -1)
        .filter(|&(_, a)| a[KEY_POMERANIANS] == 3 || a[KEY_POMERANIANS] == -1)
        .filter(|&(_, a)| a[KEY_AKITAS] == 0 || a[KEY_AKITAS] == -1)
        .filter(|&(_, a)| a[KEY_VIZSLAS] == 0 || a[KEY_VIZSLAS] == -1)
        .filter(|&(_, a)| a[KEY_GOLDFISH] == 5 || a[KEY_GOLDFISH] == -1)
        .filter(|&(_, a)| a[KEY_TREES] == 3 || a[KEY_TREES] == -1)
        .filter(|&(_, a)| a[KEY_CARS] == 3 || a[KEY_CARS] == -1)
        .filter(|&(_, a)| a[KEY_PERFUMES] == 1 || a[KEY_PERFUMES] == -1)
        // because the aunts are 1 based unlike the arrays
        .map(|(idx, _)| idx + 1)
        .next()
        .unwrap()
}

fn solve_part_two(aunts: &[[i8; 10]]) -> usize {
    aunts
        .iter()
        .enumerate()
        .filter(|&(_, a)| a[KEY_CHILDREN] == 3 || a[KEY_CHILDREN] == -1)
        .filter(|&(_, a)| a[KEY_CATS] > 7 || a[KEY_CATS] == -1)
        .filter(|&(_, a)| a[KEY_SAMOYEDS] == 2 || a[KEY_SAMOYEDS] == -1)
        .filter(|&(_, a)| a[KEY_POMERANIANS] < 3 || a[KEY_POMERANIANS] == -1)
        .filter(|&(_, a)| a[KEY_AKITAS] == 0 || a[KEY_AKITAS] == -1)
        .filter(|&(_, a)| a[KEY_VIZSLAS] == 0 || a[KEY_VIZSLAS] == -1)
        .filter(|&(_, a)| a[KEY_GOLDFISH] < 5 || a[KEY_GOLDFISH] == -1)
        .filter(|&(_, a)| a[KEY_TREES] > 3 || a[KEY_TREES] == -1)
        .filter(|&(_, a)| a[KEY_CARS] == 3 || a[KEY_CARS] == -1)
        .filter(|&(_, a)| a[KEY_PERFUMES] == 1 || a[KEY_PERFUMES] == -1)
        // because the aunts are 1 based unlike the arrays
        .map(|(idx, _)| idx + 1)
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_input(&input);
        let answer = solve_part_one(&parsed);
        assert_eq!(213, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_input(&input);
        let answer = solve_part_two(&parsed);
        assert_eq!(323, answer);
    }
}
