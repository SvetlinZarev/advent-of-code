use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_csv;
use aoc_day_06::{part_one_v1, part_two_v1};

fn main() {
    let input = parse_csv(load_text_input_from_autodetect());

    println!("Part 1: {}", part_one_v1(&input));
    println!("Part 2: {}", part_two_v1(&input));
}
