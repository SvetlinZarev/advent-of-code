use aoc_day_10::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let (points, velocities) = parse_input(input.trim())?;

    println!("Part 1: {:?}", part_one(&points, &velocities));
    println!("Part 2: {:?}", part_two(&points, &velocities));

    Ok(())
}
