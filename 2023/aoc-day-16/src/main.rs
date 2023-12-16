use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_u8_grid_borrowed;

use aoc_day_16::{part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let input = parse_u8_grid_borrowed(&input);

    println!("Part 1: {:?}", part_one(&input));
    println!("Part 2: {:?}", part_two(&input));

    Ok(())
}
