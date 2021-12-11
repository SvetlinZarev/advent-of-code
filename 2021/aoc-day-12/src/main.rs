use aoc_day_12::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;

fn main() {
    let (graph, limits) = parse_input(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(&graph, &limits));
    println!("Part 2: {}", part_two(&graph, &limits));
}
