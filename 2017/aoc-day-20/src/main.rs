use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_20::{parse_input, part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let (p, v, a) = parse_input(&input);

    println!("Part 1: {:?}", part_one(&p, &v, &a));
    println!("Part 2: {:?}", part_two(&p, &v, &a));

    Ok(())
}
