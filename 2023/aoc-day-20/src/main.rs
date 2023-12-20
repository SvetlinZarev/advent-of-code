use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_20::{load_graph, part_one};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let graph = load_graph(&input);

    println!("Part 1: {:?}", part_one(&graph));

    Ok(())
}
