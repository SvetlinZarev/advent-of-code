use aoc_day_25::part_one;
use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_u8_grid;

fn main() {
    let input = parse_u8_grid(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(input));
}
