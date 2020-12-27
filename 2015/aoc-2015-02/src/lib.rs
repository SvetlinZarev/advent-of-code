use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::min;
use aoc_2015_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: &str = "day-02";

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d_p, data) = measure(2, "parsing", || parse_input(&input));
    let (d_1, _) = measure(2, "part 1", || part_one::solve(&data));
    let (d_2, _) = measure(2, "part 2", || part_two::solve(&data));

    d_p.add(d_1).add(d_2)
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
