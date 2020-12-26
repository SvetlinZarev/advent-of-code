use std::collections::HashSet;

use crate::Direction;

pub fn solve(directions: &[Direction]) -> usize {
    let mut visited_houses = HashSet::new();
    let (mut sx, mut sy) = (0, 0);
    let (mut rx, mut ry) = (0, 0);

    visited_houses.insert((sx, sy));

    for idx in (0..directions.len() - 1).step_by(2) {
        //santa
        match directions[idx] {
            Direction::N => sy += 1,
            Direction::E => sx += 1,
            Direction::S => sy -= 1,
            Direction::W => sx -= 1,
        }

        // robo-santa
        match directions[idx + 1] {
            Direction::N => ry += 1,
            Direction::E => rx += 1,
            Direction::S => ry -= 1,
            Direction::W => rx -= 1,
        }

        visited_houses.insert((sx, sy));
        visited_houses.insert((rx, ry));
    }

    // if the input is not even, then there would be one more visit by normal santa
    if directions.len() & 1 == 1 {
        match directions[directions.len() - 1] {
            Direction::N => sy += 1,
            Direction::E => sx += 1,
            Direction::S => sy -= 1,
            Direction::W => sx -= 1,
        }
        visited_houses.insert((sx, sy));
    }

    visited_houses.len()
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn test_sample_1() {
        let input = parse_input("^v");
        let count = solve(&input);
        assert_eq!(3, count);
    }

    #[test]
    fn test_sample_2() {
        let input = parse_input("^>v<");
        let count = solve(&input);
        assert_eq!(3, count);
    }

    #[test]
    fn test_sample_3() {
        let input = parse_input("^v^v^v^v^v");
        let count = solve(&input);
        assert_eq!(11, count);
    }
}
