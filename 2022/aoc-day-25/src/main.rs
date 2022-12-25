use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_25::part_one;

fn main() {
    let input = load_text_input_from_autodetect();
    println!("Part 1: {}", part_one(&input));
}
