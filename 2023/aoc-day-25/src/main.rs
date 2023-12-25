use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_25::v2;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();

    println!("Part 1: {:?}", v2::part_one(&input));

    Ok(())
}
