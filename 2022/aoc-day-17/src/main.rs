use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_17::{part_one, part_two};

fn main() {
    let input = load_text_input_from_autodetect();
    let input = input.trim_end().as_bytes();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}
