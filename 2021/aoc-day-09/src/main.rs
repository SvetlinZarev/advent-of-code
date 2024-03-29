use aoc_day_09::{part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_u8_numeric_grid;

fn main() {
    let input = parse_u8_numeric_grid(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}
