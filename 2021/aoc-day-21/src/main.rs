use aoc_day_21::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;

fn main() {
    let (a, b) = parse_input(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(a, b));
    println!("Part 2: {}", part_two(a, b));
}
