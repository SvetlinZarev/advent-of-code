use crate::math::rotate;
use crate::solver::{calculate_corrections, calculate_distances};
use crate::{Point, Set};

pub fn part_one(input: &[Vec<Point>]) -> usize {
    assert!(!input.is_empty());
    let distances = calculate_distances(input);
    let corrections = calculate_corrections(distances, input);

    let mut fixed = Set::<Point>::default();
    for (idx, scanner) in input.iter().enumerate() {
        let (rot, diff) = corrections[idx];

        for &p in scanner {
            let p = rotate(p, rot);
            fixed.insert((p.0 - diff.0, p.1 - diff.1, p.2 - diff.2));
        }
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
