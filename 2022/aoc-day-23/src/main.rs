use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_23::{parse_input, part_one, part_two_v1};

fn main() {
    let input = load_text_input_from_autodetect();
    let parsed = parse_input(input);

    println!("Part 1: {}", part_one(&parsed));
    println!("Part 2: {}", part_two_v1(&parsed));
}
