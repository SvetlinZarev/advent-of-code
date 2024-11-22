use aoc_day_08::{parse_input, part_one_iter, part_two_iter};
use aoc_shared::input::load_text_input_from_autodetect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let parsed = parse_input(&input)?;

    println!("Part 1: {:?}", part_one_iter(&parsed));
    println!("Part 2: {:?}", part_two_iter(&parsed));

    Ok(())
}
