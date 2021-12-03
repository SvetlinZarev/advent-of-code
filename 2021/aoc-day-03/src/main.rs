use aoc_shared::input::stdin_line_delimited;
use aoc_day_03::{part_one, part_two_v1};

fn main() {
    let mut input: Vec<String> = stdin_line_delimited();
    println!("Part 1: {}", part_one(&mut input));
    println!("Part 2: {}", part_two_v1(&mut input));
}
