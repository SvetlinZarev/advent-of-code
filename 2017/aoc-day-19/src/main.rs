use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_u8_grid;

use aoc_day_19::solve;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let input = parse_u8_grid(input);

    let (part_one, part_two) = solve(&input);

    println!("Part 1: {:?}", part_one);
    println!("Part 2: {:?}", part_two);

    Ok(())
}
