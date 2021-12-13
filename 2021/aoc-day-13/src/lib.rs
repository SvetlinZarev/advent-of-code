mod input;
mod parsing;

pub use input::{Fold, Point};
pub use parsing::parse_input;
use std::collections::HashSet;

pub fn part_one(points: &[Point], instr: &[Fold]) -> usize {
    let mut points = points.to_vec();

    for f_op in instr.iter().copied().take(1) {
        points.iter_mut().for_each(|p| *p = f_op.apply(*p));
    }

    points.iter().collect::<HashSet<_>>().len()
}

pub fn part_two(points: &[Point], instr: &[Fold]) -> Vec<Vec<char>> {
    let mut points = points.to_vec();

    for f_op in instr.iter().copied() {
        points.iter_mut().for_each(|p| *p = f_op.apply(*p));
    }

    let (max_x, max_y) = points
        .iter()
        .fold((0, 0), |(x, y), p| (x.max(p.x), y.max(p.y)));

    let mut grid = vec![vec![' '; (max_x + 1) as usize]; (max_y + 1) as usize];

    points
        .iter()
        .for_each(|p| grid[p.y as usize][p.x as usize] = '#');

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let (points, instr) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&points, &instr);
        assert_eq!(621, answer);
    }

    #[test]
    fn test_part_two() {
        let expected = vec![
            "#  # #  # #  #   ##  ##   ##    ## ####",
            "#  # # #  #  #    # #  # #  #    #    #",
            "#### ##   #  #    # #    #  #    #   # ",
            "#  # # #  #  #    # # ## ####    #  #  ",
            "#  # # #  #  # #  # #  # #  # #  # #   ",
            "#  # #  #  ##   ##   ### #  #  ##  ####",
        ];

        let (points, instr) = parse_input(load_text_input_from_file("inputs/input.txt"));
        let answer = part_two(&points, &instr);

        let answer = answer
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>();

        assert_eq!(expected, answer);
    }
}
