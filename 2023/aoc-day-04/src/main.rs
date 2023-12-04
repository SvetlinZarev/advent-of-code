use std::error::Error;

use aoc_shared::input::load_line_delimited_input_from_autodetect;

use aoc_day_04::{part_one, part_two_v1};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_line_delimited_input_from_autodetect();

    println!("Part 1: {:?}", part_one(&input));
    println!("Part 2: {:?}", part_two_v1(&input));

    Ok(())
}
