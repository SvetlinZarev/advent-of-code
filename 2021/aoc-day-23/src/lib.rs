mod parsing;
mod solver;

const FREE_SPOT: u8 = u8::MAX;

pub const HALL_LEN: usize = 11;
pub const ROOMS: usize = 4;

pub type Hall = [u8; HALL_LEN];
pub type Rooms<const N: usize> = [[u8; N]; ROOMS];

pub use parsing::parse_input;

pub fn part_one(input: Rooms<2>, hall: Hall) -> u64 {
    solver::solve(input, hall)
}

pub fn part_two(input: Rooms<4>, hall: Hall) -> u64 {
    solver::solve(input, hall)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let (rooms, hall) = parse_input(load_text_input_from_file("inputs/input-1.txt"));
        let answer = part_one(rooms, hall);
        assert_eq!(13556, answer);
    }

    #[test]
    fn test_part_two() {
        let (rooms, hall) = parse_input(load_text_input_from_file("inputs/input-2.txt"));
        let answer = part_two(rooms, hall);
        assert_eq!(54200, answer);
    }
}
