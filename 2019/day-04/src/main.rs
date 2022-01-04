use aoc_shared_2019::input::load_text_input_from_autodetect;
use day_04::{parse_input, part_one, part_two};

fn main() {
    let (lo, hi) = parse_input(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(lo, hi));
    println!("Part 2: {}", part_two(lo, hi));
}
