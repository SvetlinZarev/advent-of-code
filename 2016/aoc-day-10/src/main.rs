use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_10::{parse_input, part_one, part_two};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let (start_node, graph) = parse_input(&input);

    println!("Part 1: {:?}", part_one(start_node, graph.clone()));
    println!("Part 2: {:?}", part_two(start_node, graph.clone()));

    Ok(())
}
