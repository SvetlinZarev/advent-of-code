use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_05::{parse_input, part_one_v2, part_two};

fn main() {
    let (stacks, instructions) = parse_input(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one_v2(&stacks, &instructions));
    println!("Part 2: {}", part_two(&stacks, &instructions));
}
