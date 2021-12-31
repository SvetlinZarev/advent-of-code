use crate::solver::{calculate_corrections, fix_point};
use crate::{Point, Set};

pub fn part_one(input: &[Vec<Point>]) -> usize {
    assert!(!input.is_empty());
    let corrections = calculate_corrections(input);

    let mut fixed = Set::<Point>::default();
    for (idx, scanner) in input.iter().enumerate() {
        let (rot, diff) = corrections[idx];

        scanner.iter().copied().for_each(|p| {
            fixed.insert(fix_point(p, diff, rot));
        });
    }

    fixed.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&input);
        assert_eq!(378, answer);
    }
}
