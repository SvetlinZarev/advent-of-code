use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_09::part_one_and_two;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();

    let (score, garbage) = part_one_and_two(input.trim());
    println!("Part 1: {}", score);
    println!("Part 2: {}", garbage);

    Ok(())
}
