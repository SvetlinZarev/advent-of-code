use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_csv;

use aoc_day_10::{part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let input_parsed = parse_csv(&input);

    println!("Part 1: {:?}", part_one(&input_parsed));
    println!("Part 2: {:?}", part_two(input.trim_end()));

    Ok(())
}
