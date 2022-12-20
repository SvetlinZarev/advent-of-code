use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_16::{p1v2, p2v2, parse_input};

fn main() {
    let input = load_text_input_from_autodetect();
    let graph = parse_input(input);

    println!("Part 1: {}", p1v2::part_one(&graph));
    println!("Part 2: {}", p2v2::part_two(&graph));
}
