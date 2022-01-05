use aoc_shared_2019::hashing::{FxHasher, HashBuilder};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type Set<K> = HashSet<K, HashBuilder<FxHasher>>;
type Map<K, V> = HashMap<K, V, HashBuilder<FxHasher>>;

const CENTER_OF_MASS: &'static str = "COM";
const SANTA: &'static str = "SAN";
const YOU: &'static str = "YOU";

pub fn parse_input(input: &str) -> Map<&str, Vec<&str>> {
    let mut centers_of_mass = Set::default();
    let mut orbiters = Set::default();
    let mut orbits = Map::default();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (com, orbiter) = line.split_once(')').unwrap();

        centers_of_mass.remove(orbiter);
        orbiters.insert(orbiter);
        match orbiters.contains(com) {
            true => centers_of_mass.remove(com),
            false => centers_of_mass.insert(com),
        };

        orbits
            .entry(com)
            .and_modify(|v: &mut Vec<_>| v.push(orbiter))
            .or_insert(vec![orbiter]);
    }

    assert_eq!(centers_of_mass.len(), 1);
    assert!(centers_of_mass.contains(CENTER_OF_MASS));
    orbits
}

pub fn part_one(orbits: &Map<&str, Vec<&str>>) -> u64 {
    let mut total = 0;

    let mut queue = VecDeque::new();
    queue.push_back((CENTER_OF_MASS, 0));

    while let Some((center_of_mass, number_of_orbits)) = queue.pop_front() {
        for orbiters in orbits.get(center_of_mass) {
            for orbit in orbiters.iter().copied() {
                queue.push_back((orbit, number_of_orbits + 1));
                total += number_of_orbits + 1;
            }
        }
    }

    total
}

pub fn part_two(orbits: &Map<&str, Vec<&str>>) -> u64 {
    let reversed_map = reverse_mapping(orbits);
    let &starting_point = reversed_map.get(YOU).unwrap();
    let &destination = reversed_map.get(SANTA).unwrap();

    let mut visited = Set::default();
    visited.insert(YOU);

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), starting_point));

    while let Some((Reverse(score), com)) = queue.pop() {
        if com == destination {
            return score;
        }

        visited.insert(com);
        let transfers = score + 1;

        if let Some(orbiters) = orbits.get(com) {
            for orbiter in orbiters {
                if !visited.contains(orbiter) {
                    queue.push((Reverse(transfers), orbiter))
                }
            }
        }

        if let Some(parent) = reversed_map.get(com) {
            if !visited.contains(parent) {
                queue.push((Reverse(transfers), parent))
            }
        }
    }

    panic!("There is no path between you and Santa!");
}

fn reverse_mapping<'l>(orbits: &Map<&'l str, Vec<&'l str>>) -> Map<&'l str, &'l str> {
    let mut reversed = Map::default();
    for (&com, orbiters) in orbits.iter() {
        for &orbiter in orbiters {
            reversed.insert(orbiter, com);
        }
    }

    reversed
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared_2019::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let raw_input = load_text_input_from_file("inputs/input.txt");
        let orbits = parse_input(&raw_input);

        let answer = part_one(&orbits);
        assert_eq!(268504, answer);
    }

    #[test]
    fn test_part_two() {
        let raw_input = load_text_input_from_file("inputs/input.txt");
        let orbits = parse_input(&raw_input);

        let answer = part_two(&orbits);
        assert_eq!(409, answer);
    }
}
