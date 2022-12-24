use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_24::parse_input;
use aoc_day_24::{part_one, part_two};

fn main() {
    let input = load_text_input_from_autodetect();
    let (grid, start, end) = parse_input(input);

    println!("Part 1: {}", part_one(&grid, start, end));
    println!("Part 2: {}", part_two(&grid, start, end));
}
