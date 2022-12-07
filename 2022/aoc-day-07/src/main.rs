use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_07::{parse_input, v1};

fn main() {
    let input = load_text_input_from_autodetect();
    let entries = parse_input(&input);

    println!("Part 1: {}", v1::part_one(&entries));
    println!("Part 2: {}", v1::part_two(&entries));
}
