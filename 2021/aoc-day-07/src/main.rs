use aoc_day_07::{part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_csv;

fn main() {
    let input = parse_csv(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}
