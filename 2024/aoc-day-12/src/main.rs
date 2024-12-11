use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_12::{part_one_v1, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();

    println!("Part 1: {:?}", part_one_v1(&input));
    println!("Part 2: {:?}", part_two(&input));

    Ok(())
}
