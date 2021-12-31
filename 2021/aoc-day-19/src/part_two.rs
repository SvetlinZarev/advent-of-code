use crate::solver::calculate_corrections;
use crate::{Int, Point};

pub fn part_two(input: &[Vec<Point>]) -> Int {
    assert!(!input.is_empty());
    let corrections = calculate_corrections(input);

    let mut best = 0;
    for i in 0..corrections.len() {
        for j in i + 1..corrections.len() {
            let (_, a) = &corrections[i];
            let (_, b) = &corrections[j];

            let dist = (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs();
            best = best.max(dist);
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_input;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_two() {
        let input = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&input);
        assert_eq!(13148, answer);
    }
}
