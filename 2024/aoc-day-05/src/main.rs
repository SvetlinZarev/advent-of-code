use std::error::Error;

use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_05::{parse_input, part_one, part_two_topo_sort};

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_text_input_from_autodetect();
    let (graph, updates) = parse_input(&input)?;

    println!("Part 1: {:?}", part_one(&graph, &updates));
    println!("Part 2: {:?}", part_two_topo_sort(&graph, &updates));

    Ok(())
}
