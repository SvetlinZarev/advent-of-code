use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: &'static str = "day-06";

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (d_p, instructions) = measure(6, "parsing", || parse_input(&input));
    let (d_1, _) = measure(6, "part 1", || part_one::solve(&instructions));
    let (d_2, _) = measure(6, "part 2", || part_two::solve(&instructions));

    d_p.add(d_1).add(d_2)
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    from: Coordinate,
    to: Coordinate,
    action: Action,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Coordinate {
    x: u16,
    y: u16,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    On,
    Off,
    Toggle,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    for line in input.lines() {
        let action;
        let mut line = line;

        if line.starts_with("toggle") {
            action = Action::Toggle;
            line = &line[7..];
        } else if line.starts_with("turn on") {
            action = Action::On;
            line = &line[8..];
        } else if line.starts_with("turn off") {
            action = Action::Off;
            line = &line[9..];
        } else {
            panic!("Unrecognized instruction: {}", line);
        }

        let x_end = line.find(',').unwrap();
        let x = line[..x_end].parse().unwrap();
        line = &line[x_end + 1..];

        let y_end = line.find(' ').unwrap();
        let y = line[..y_end].parse().unwrap();
        line = &line[y_end + 1 + 8..];

        let from = Coordinate { x, y };

        let x_end = line.find(',').unwrap();
        let x = line[..x_end].parse().unwrap();
        line = &line[x_end + 1..];

        let y = line.parse().unwrap();

        let to = Coordinate { x, y };
        let instruction = Instruction { from, to, action };

        instructions.push(instruction);
    }

    instructions
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_parse_input_off() {
        let instruction = parse_input("turn off 30,309 through 259,554")[0];
        assert_eq!(Action::Off, instruction.action);
        assert_eq!(Coordinate { x: 30, y: 309 }, instruction.from);
        assert_eq!(Coordinate { x: 259, y: 554 }, instruction.to);
    }

    #[test]
    fn test_parse_input_on() {
        let instruction = parse_input("turn on 809,221 through 869,723")[0];
        assert_eq!(Action::On, instruction.action);
        assert_eq!(Coordinate { x: 809, y: 221 }, instruction.from);
        assert_eq!(Coordinate { x: 869, y: 723 }, instruction.to);
    }

    #[test]
    fn test_parse_input_toggle() {
        let instruction = parse_input("toggle 492,660 through 647,910")[0];
        assert_eq!(Action::Toggle, instruction.action);
        assert_eq!(Coordinate { x: 492, y: 660 }, instruction.from);
        assert_eq!(Coordinate { x: 647, y: 910 }, instruction.to);
    }

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let instructions = parse_input(&input);
        let solution = part_one::solve(&instructions);
        assert_eq!(543903, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let instructions = parse_input(&input);
        let solution = part_two::solve(&instructions);
        assert_eq!(14687245, solution);
    }
}
