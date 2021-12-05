use aoc_shared::input::load_line_delimited_input_from_autodetect;
use aoc_day_03::{part_one, part_two_v1};

fn main() {
    let mut input = load_line_delimited_input_from_autodetect();
    println!("Part 1: {}", part_one(&mut input));
    println!("Part 2: {}", part_two_v1(&mut input));
}
