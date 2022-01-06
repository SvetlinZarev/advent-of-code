use aoc_shared_2019::input::load_text_input_from_autodetect;
use day_08::{parse_input, part_one, part_two, COLS, ROWS};

fn main() {
    let input = parse_input(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(&input, ROWS, COLS));
    println!("Part 2:\n{}", part_two(&input, ROWS, COLS));
}
