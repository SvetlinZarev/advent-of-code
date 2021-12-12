use aoc_day_12::{parse_input, part_one_v2, part_two_v2, simplify_graph};
use aoc_shared::input::load_text_input_from_autodetect;

fn main() {
    let (graph, limits) = parse_input(load_text_input_from_autodetect());
    let graph = simplify_graph(&graph, &limits);

    println!("Part 1: {}", part_one_v2(&graph));
    println!("Part 2: {}", part_two_v2(&graph));
}
