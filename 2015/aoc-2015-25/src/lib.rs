use std::path::Path;
use std::time::Duration;

use regex::Regex;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

const DAY: usize = 25;
const INITIAL: u64 = 20151125;
const MUL: u64 = 252533;
const DIV: u64 = 33554393;


pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let (row, col) = parse_input(load_input(path));
    let (d1, _) = measure(DAY, "part 1", || solve_part_one(row, col));

    d1
}

fn parse_input(input: String) -> (u64, u64) {
    let regex = Regex::new(r#".+row (?<row>\d+), column (?<column>\d+)\."#).unwrap();
    let captures = regex.captures(&input).unwrap();

    let row = captures.name("row").unwrap();
    let row = row.as_str().parse().unwrap();

    let col = captures.name("column").unwrap();
    let col = col.as_str().parse().unwrap();


    (row, col)
}

fn solve_part_one(row: u64, col: u64) -> u64 {
    let mut r = 1;
    let mut c = 1;
    let mut x = INITIAL;

    loop {
        if r == 1 {
            r = c + 1;
            c = 1;
        } else {
            r -= 1;
            c += 1;
        }

        x = (x * MUL) % DIV;
        if r == row && c == col {
            break;
        }
    }

    x
}

#[cfg(test)]
mod tests {
    use crate::solve_part_one;

    #[test]
    fn test_part_one() {
        let actual = solve_part_one(2981, 3075);
        assert_eq!(9132360, actual);
    }
}
