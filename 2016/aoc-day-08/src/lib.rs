use regex::Regex;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Instruction {
    Rect(usize, usize),
    RotRow(usize, usize),
    RotCol(usize, usize),
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let rect = Regex::new(r#"rect (?<width>\d+)x(?<height>\d+)"#).unwrap();
    let rot_row = Regex::new(r#"rotate row y=(?<row>\d+) by (?<amount>\d+)"#).unwrap();
    let rot_col = Regex::new(r#"rotate column x=(?<column>\d+) by (?<amount>\d+)"#).unwrap();

    let mut answer = vec![];
    for line in input.lines() {
        if let Some(cap) = rect.captures(line) {
            let width = cap.name("width").unwrap();
            let width = width.as_str().parse().unwrap();

            let amount = cap.name("height").unwrap();
            let amount = amount.as_str().parse().unwrap();

            answer.push(Instruction::Rect(width, amount));
        } else if let Some(cap) = rot_row.captures(line) {
            let row = cap.name("row").unwrap();
            let row = row.as_str().parse().unwrap();

            let amount = cap.name("amount").unwrap();
            let amount = amount.as_str().parse().unwrap();

            answer.push(Instruction::RotRow(row, amount));
        } else if let Some(cap) = rot_col.captures(line) {
            let column = cap.name("column").unwrap();
            let column = column.as_str().parse().unwrap();

            let amount = cap.name("amount").unwrap();
            let amount = amount.as_str().parse().unwrap();

            answer.push(Instruction::RotCol(column, amount));
        } else {
            panic!("invalid input: {}", line)
        }
    }

    answer
}

const EMPTY: char = ' ';
const FULL_BLOCK: char = '█';

pub fn part_one(input: &[Instruction]) -> usize {
    let screen = process_input(input);

    screen
        .iter()
        .flat_map(|row| row.iter().copied())
        .filter(|&x| x == FULL_BLOCK)
        .count()
}

pub fn part_two(input: &[Instruction]) -> String {
    let screen = process_input(input);
    screen
        .iter()
        .flat_map(|row| row.iter().copied().chain(std::iter::once('\n')))
        .collect()
}

fn process_input(input: &[Instruction]) -> [[char; 50]; 6] {
    let mut screen = [[EMPTY; WIDTH]; HEIGHT];

    for instruction in input.iter().copied() {
        match instruction {
            Instruction::Rect(width, height) => {
                for r in 0..height {
                    for c in 0..width {
                        screen[r][c] = FULL_BLOCK
                    }
                }
            }

            Instruction::RotRow(row, amount) => {
                screen[row].rotate_right(amount);
            }

            Instruction::RotCol(col, amount) => {
                let mut column = [EMPTY; HEIGHT];

                for r in 0..screen.len() {
                    column[r] = screen[r][col]
                }

                column.rotate_right(amount);

                for r in 0..screen.len() {
                    screen[r][col] = column[r];
                }
            }
        }
    }
    screen
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{parse_input, part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input);
        let answer = part_one(&input);
        assert_eq!(123, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let input = parse_input(&input);
        let answer = part_two(&input);
        assert_eq!(
            " ██  ████ ███  █  █ ███  ████ ███    ██ ███   ███ \n\
             █  █ █    █  █ █  █ █  █    █ █  █    █ █  █ █    \n\
             █  █ ███  ███  █  █ █  █   █  ███     █ █  █ █    \n\
             ████ █    █  █ █  █ ███   █   █  █    █ ███   ██  \n\
             █  █ █    █  █ █  █ █    █    █  █ █  █ █       █ \n\
             █  █ █    ███   ██  █    ████ ███   ██  █    ███  \n",
            answer
        );
    }
}
