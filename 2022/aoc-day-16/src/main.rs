use aoc_shared::input::load_text_input_from_stdin;

use aoc_day_16::{p1v1, p2v1, parse_input};

fn main() {
    let input = load_text_input_from_stdin();
    let graph = parse_input(input);

    println!("Part 1: {}", p1v1::part_one(&graph));
    println!("Part 2: {}", p2v1::part_two(&graph));
}
