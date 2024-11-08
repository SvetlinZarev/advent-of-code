use aoc_day_03::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let parsed = parse_input(&input)?;

    println!("Part 1: {:?}", part_one(&parsed));
    println!("Part 2: {:?}", part_two(&parsed));

    Ok(())
}
