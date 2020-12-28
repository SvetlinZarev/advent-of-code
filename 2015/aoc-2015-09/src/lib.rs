use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

mod exhaustive;

pub const DAY: usize = 9;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (dp, routes) = measure(DAY, "parsing", || parse_input(&input));
    let (d1, _) = measure(DAY, "part 1: O(n!)", || solve_part_one(&routes));
    let (d2, _) = measure(DAY, "part 2: O(n!)", || solve_part_two(&routes));

    dp + d1 + d2
}

pub fn solve_part_one(routes: &Routes) -> u64 {
    exhaustive::solve(routes, |a: u64, b: u64| a.cmp(&b))
}

pub fn solve_part_two(routes: &Routes) -> u64 {
    exhaustive::solve(routes, |a: u64, b: u64| a.cmp(&b).reverse())
}

#[derive(Debug, Clone)]
pub struct Routes {
    dist: Vec<u64>,
    cities: usize,
}

impl Routes {
    pub fn new(cities: usize, dist: Vec<u64>) -> Routes {
        assert_eq!(cities * cities, dist.len());
        Routes { cities, dist }
    }

    pub fn distance(&self, from: usize, to: usize) -> u64 {
        self.dist[from * self.cities + to]
    }
}

pub fn parse_input(input: &str) -> Routes {
    let mut routes = HashMap::new();
    let mut city_idxs = HashMap::new();

    for line in input.lines() {
        let mut line = line;
        let mut idx = line.find(' ').unwrap();
        let from = &line[..idx];

        line = &line[idx + 4..];
        idx = line.find(' ').unwrap();
        let to = &line[..idx];

        line = &line[idx + 3..];
        let dist = line.parse().unwrap();

        let a = from.min(to);
        let b = from.max(to);
        routes.insert((a, b), dist);

        let mut city_idx = city_idxs.len();
        city_idxs.entry(a).or_insert(city_idx);

        city_idx = city_idxs.len();
        city_idxs.entry(b).or_insert(city_idx);
    }

    let cities = city_idxs.len();
    let mut dist = vec![0; cities.pow(2)];

    for (&c1, &i) in city_idxs.iter() {
        for (&c2, &j) in city_idxs.iter() {
            if i != j {
                let key = if c1 < c2 { (c1, c2) } else { (c2, c1) };
                let d = *routes.get(&key).unwrap();

                dist[i * cities + j] = d;
                dist[j * cities + i] = d;
            }
        }
    }

    Routes { dist, cities }
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let routes = parse_input(&input);
        let answer = solve_part_one(&routes);

        assert_eq!(117, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let routes = parse_input(&input);
        let answer = solve_part_two(&routes);

        assert_eq!(909, answer);
    }
}
