use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_17::{part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();

    println!("Part 1: {:?}", part_one(input.as_bytes()));
    println!("Part 2: {:?}", part_two(input.as_bytes()));

    Ok(())
}
