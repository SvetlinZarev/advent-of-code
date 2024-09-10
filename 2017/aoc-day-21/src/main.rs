use aoc_day_21::{part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();

    println!("Part 1: {:?}", part_one(&input));
    println!("Part 2: {:?}", part_two(&input));

    Ok(())
}
