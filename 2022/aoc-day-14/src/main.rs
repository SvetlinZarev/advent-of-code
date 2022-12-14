use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_14::{part_one, part_two_v1};
use aoc_day_14::parse_input;

fn main() {
    let input = load_text_input_from_autodetect();
    let (grid, last_row, initial_column) = parse_input(input);

    println!("Part 1: {}", part_one(grid.clone(), last_row, initial_column));
    println!("Part 2: {}", part_two_v1(grid, initial_column));
}
