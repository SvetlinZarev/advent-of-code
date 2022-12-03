use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Section {
    from: u32,
    to: u32,
}

impl Section {
    pub fn contains(self, other: Section) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    pub fn overlap(self, other: Section) -> bool {
        (other.from >= self.from && other.from <= self.to)
            || (self.from >= other.from && self.from <= other.to)
    }
}

impl FromStr for Section {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once('-')
            .ok_or_else(|| format!("cannot split section: {}", s))?;

        Ok(Self {
            from: a.parse().map_err(|_| format!("cannot parse: {}", a))?,
            to: b.parse().map_err(|_| format!("cannot parse: {}", b))?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Assignment {
    first: Section,
    second: Section,
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(',')
            .ok_or_else(|| format!("cannot split assignment: {}", s))?;

        Ok(Self {
            first: a.parse()?,
            second: b.parse()?,
        })
    }
}

pub fn part_one(input: &[Assignment]) -> usize {
    input
        .iter()
        .filter(|&ass| ass.first.contains(ass.second) || ass.second.contains(ass.first))
        .count()
}

pub fn part_two(input: &[Assignment]) -> usize {
    input
        .iter()
        .filter(|&ass| ass.first.overlap(ass.second))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);
        assert_eq!(538, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(792, answer);
    }
}
