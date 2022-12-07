use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_07::{parse_input, part_one, part_two};

fn main() {
    let input = load_text_input_from_autodetect();
    let entries = parse_input(&input);

    println!("Part 1: {}", part_one(&entries));
    println!("Part 2: {}", part_two(&entries));
}
