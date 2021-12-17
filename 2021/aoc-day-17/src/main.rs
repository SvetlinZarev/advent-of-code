use aoc_day_17::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;

fn main() {
    let (x0, x1, y0, y1) = parse_input(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(x0, x1, y0, y1));
    println!("Part 2: {}", part_two(x0, x1, y0, y1));
}
