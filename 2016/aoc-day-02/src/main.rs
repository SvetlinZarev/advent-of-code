use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_02::{part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    println!("Part 1: {:?}", part_one(&input));
    println!("Part 2: {:?}", part_two(&input));

    Ok(())
}
