use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::parsing::parse_lines_as_i32;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 1;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let input = parse_lines_as_i32(&input);

    measure(DAY, "part 1 / bruteforce", || {
        part_one::solve_bruteforce(&input)
    });

    let (d_1, _) = measure(DAY, "part 1 / sorting", || {
        part_one::solve_with_sorting(&input)
    });

    measure(DAY, "part 2 / bruteforce", || {
        part_two::solve_with_bruteforce(&input)
    });

    let (d_2, _) = measure(DAY, "part 2 / O(N^2)", || {
        part_two::solve_with_quadratic_alg(&input)
    });

    d_1.add(d_2)
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let input = parse_lines_as_i32(&input);

        let solution = part_one::solve_bruteforce(&input);
        assert_eq!(Some(1020036), solution);

        let solution = part_one::solve_with_sorting(&input);
        assert_eq!(Some(1020036), solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let input = parse_lines_as_i32(&input);

        let solution = part_two::solve_with_bruteforce(&input);
        assert_eq!(Some(286977330), solution);

        let solution = part_two::solve_with_quadratic_alg(&input);
        assert_eq!(Some(286977330), solution);
    }
}
