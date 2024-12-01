use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_01::{parse_input_generic, part_one, part_two_v1};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let (a, b) = parse_input_generic(&input)?;

    println!("Part 1: {:?}", part_one(&a, &b));
    println!("Part 2: {:?}", part_two_v1(&a, &b));

    Ok(())
}
