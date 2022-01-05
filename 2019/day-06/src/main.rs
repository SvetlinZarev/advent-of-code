use aoc_shared_2019::input::load_text_input_from_autodetect;
use day_06::{parse_input, part_one, part_two};

fn main() {
    let raw_input = load_text_input_from_autodetect();
    let orbits = parse_input(&raw_input);

    println!("Part 1: {}", part_one(&orbits));
    println!("Part 2: {}", part_two(&orbits));
}
