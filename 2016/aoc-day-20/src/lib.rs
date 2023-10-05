use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Range(u32, u32);

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s
            .split_once('-')
            .ok_or_else(|| format!("invalid range: {}", s))?;

        Ok(Range(
            from.parse().map_err(|_| format!("invalid range: {}", s))?,
            to.parse().map_err(|_| format!("invalid range: {}", s))?,
        ))
    }
}

pub fn part_one(input: &[Range]) -> u32 {
    let allowed = split_to_intervals(input);
    allowed.first_key_value().map(|(&from, _)| from).unwrap()
}

pub fn part_two(input: &[Range]) -> u32 {
    let allowed = split_to_intervals(input);
    allowed.iter().map(|(&from, &to)| to - from + 1).sum()
}

fn split_to_intervals(input: &[Range]) -> BTreeMap<u32, u32> {
    let mut allowed = BTreeMap::new();
    allowed.insert(0, u32::MAX);

    for range in input.iter().copied() {
        while let Some((&begin, &end)) = allowed.range(..=range.1).last() {
            // The intervals do not intersect
            if end < range.0 {
                break;
            }

            // The intervals intersect, so we have to split it
            allowed.remove(&begin);

            if begin < range.0 {
                allowed.insert(begin, range.0 - 1);
            }

            if end > range.1 {
                allowed.insert(range.1 + 1, end);
            }
        }
    }
    allowed
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(22887907, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(109, answer);
    }
}
