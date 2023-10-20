use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_12::part_one_and_two;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();

    let (one, two) = part_one_and_two(&input);
    println!("Part 1: {:?}", one);
    println!("Part 2: {:?}", two);

    Ok(())
}
