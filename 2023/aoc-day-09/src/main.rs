use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_09::{parse_input, part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let input = parse_input(&input);

    println!("Part 1: {:?}", part_one(&input));
    println!("Part 2: {:?}", part_two(&input));

    Ok(())
}
