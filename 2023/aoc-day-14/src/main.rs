use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_u8_grid;

use aoc_day_14::{part_one, part_two_v2};

fn main() -> Result<(), Box<dyn Error>> {
    let input_text = load_text_input_from_autodetect();
    let input = parse_u8_grid(&input_text);

    println!("Part 1: {:?}", part_one(&input));
    println!("Part 2: {:?}", part_two_v2(input_text.as_bytes()));

    Ok(())
}
