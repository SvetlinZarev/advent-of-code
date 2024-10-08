use aoc_day_25::{parse_input, part_one_v1};
use aoc_shared::input::load_text_input_from_autodetect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let input = parse_input(&input)?;

    println!("Part 1: {:?}", part_one_v1(&input));

    Ok(())
}
