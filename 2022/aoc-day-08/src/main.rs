use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_u8_numeric_grid;

use aoc_day_08::{part_one, part_two};

fn main() {
    let input = load_text_input_from_autodetect();
    let grid = parse_u8_numeric_grid(&input);

    println!("Part 1: {}", part_one(&grid));
    println!("Part 2: {}", part_two(&grid));
}
