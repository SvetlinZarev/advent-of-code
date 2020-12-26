use std::collections::HashSet;

use crate::Direction;

pub fn solve(directions: &[Direction]) -> usize {
    let mut visited_houses = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    // we should count the starting point too
    visited_houses.insert((y, x));
    for d in directions {
        match d {
            Direction::N => y += 1,
            Direction::E => x += 1,
            Direction::S => y -= 1,
            Direction::W => x -= 1,
        }

        visited_houses.insert((y, x));
    }

    visited_houses.len()
}

#[cfg(test)]
mod tests {
    use crate::parse_input;

    use super::*;

    #[test]
    fn test_sample_1() {
        let input = parse_input(">");
        let count = solve(&input);
        assert_eq!(2, count);
    }

    #[test]
    fn test_sample_2() {
        let input = parse_input("^>v<");
        let count = solve(&input);
        assert_eq!(4, count);
    }

    #[test]
    fn test_sample_3() {
        let input = parse_input("^v^v^v^v^v");
        let count = solve(&input);
        assert_eq!(2, count);
    }
}
