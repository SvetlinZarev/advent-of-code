use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_csv;

use aoc_day_11::{part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let parsed = parse_csv(input);

    println!("Part 1: {:?}", part_one(&parsed));
    println!("Part 2: {:?}", part_two(&parsed));

    Ok(())
}
