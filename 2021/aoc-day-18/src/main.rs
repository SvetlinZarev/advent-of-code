use aoc_day_18::{part_one, part_two, Number};
use aoc_shared::input::load_line_delimited_input_from_autodetect;

fn main() {
    let input: Vec<Number> = load_line_delimited_input_from_autodetect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}
