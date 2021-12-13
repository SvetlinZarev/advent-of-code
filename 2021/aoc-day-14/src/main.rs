use aoc_day_14::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;

fn main() {
    let (polymer, rules) = parse_input(load_text_input_from_autodetect());

    println!("Part 1: {}", part_one(&polymer, &rules));
    println!("Part 2: {}", part_two(&polymer, &rules));
}
