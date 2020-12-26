use std::path::Path;

use aoc_2015_common::input::load_input;
use aoc_2015_common::min;
use aoc_2015_common::output::measure_solution;

pub mod part_one;
pub mod part_two;

pub const DAY: &str = "day-02";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let data = parse_input(&input);

    measure_solution(2, 1, "", || part_one::solve(&data));
    measure_solution(2, 2, "", || part_two::solve(&data));
}

#[derive(Debug, Copy, Clone)]
pub struct Cuboid {
    l: u32,
    w: u32,
    h: u32,
}

impl Cuboid {
    pub fn new(l: u32, w: u32, h: u32) -> Cuboid {
        Cuboid { l, w, h }
    }

    pub fn area(self) -> u32 {
        0 + 2 * (self.l * self.w) + 2 * (self.w * self.h) + 2 * (self.h * self.l)
    }

    pub fn min_side_area(self) -> u32 {
        let a = self.l * self.w;
        let b = self.w * self.h;
        let c = self.h * self.l;
        min!(a, b, c)
    }

    pub fn volume(self) -> u32 {
        self.l * self.w * self.h
    }

    pub fn min_side_perimeter(self) -> u32 {
        let a = min!(self.w + self.h);
        let b = min!(self.w + self.l);
        let c = min!(self.h + self.l);
        min!(a, b, c) * 2
    }
}

pub fn parse_input(input: &str) -> Vec<Cuboid> {
    input
        .lines()
        .map(|l| l.split('x'))
        .map(|mut s| {
            Cuboid::new(
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let data = parse_input(&input);

        let solution = part_one::solve(&data);
        assert_eq!(1588178, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let data = parse_input(&input);

        let solution = part_two::solve(&data);
        assert_eq!(3783758, solution);
    }
}
