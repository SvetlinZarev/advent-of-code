use aoc_day_17::{part_one_deque, part_one_list, part_two_deque, part_two_idxs};
use aoc_shared::input::load_text_input_from_autodetect;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let value = input.trim().parse()?;

    println!("Part 1: {:?}", part_one_deque(value));
    println!("Part 2: {:?}", part_two_idxs(value));

    Ok(())
}
