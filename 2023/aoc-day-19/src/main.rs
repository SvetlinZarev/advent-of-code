use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_19::{parse_input, part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let (rules, data) = parse_input(&input, true).unwrap();

    println!("Part 1: {:?}", part_one(&rules, &data));
    println!("Part 2: {:?}", part_two(&rules));

    Ok(())
}
