use aoc_day_02::{part_one, part_two, Direction};
use aoc_shared::input::stdin_line_delimited;

fn main() {
    let input: Vec<Direction> = stdin_line_delimited();
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}
