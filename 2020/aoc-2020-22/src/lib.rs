use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::parsing::parse_lines_as_usize;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 22;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (first, second) = parse_input(&input);

    let (d1, _) = measure(DAY, "part 1", || part_one::solve(&first, &second));
    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&first, &second));

    d1.add(d2)
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let separator = input.find("\n\n").unwrap();
    let (a, b) = input.split_at(separator);

    let player_one = parse_lines_as_usize(a[9..].trim());
    let player_two = parse_lines_as_usize(b[9 + 2..].trim());

    (player_one, player_two)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2020_common::input::default_test_input;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let (a, b) = parse_input(&input);

        let solution = part_one::solve(&a, &b);
        assert_eq!(32401, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let (a, b) = parse_input(&input);

        let solution = part_two::solve(&a, &b);
        assert_eq!(31436, solution);
    }
}
