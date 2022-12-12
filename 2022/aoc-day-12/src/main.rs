use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_u8_grid;

use aoc_day_12::{part_one_v2, part_two_v2};

fn main() {
    let input = load_text_input_from_autodetect();
    let grid = parse_u8_grid(&input);

    println!("Part 1: {}", part_one_v2(grid.clone()));
    println!("Part 2: {}", part_two_v2(grid));
}
