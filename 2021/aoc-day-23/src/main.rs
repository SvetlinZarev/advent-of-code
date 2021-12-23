use aoc_day_23::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_file;

fn main() {
    let (rooms, hall) = parse_input(load_text_input_from_file("inputs/input-1.txt"));
    println!("Part 1: {}", part_one(rooms, hall));

    let (rooms, hall) = parse_input(load_text_input_from_file("inputs/input-1.txt"));
    println!("Part 2: {}", part_two(rooms, hall));
}
