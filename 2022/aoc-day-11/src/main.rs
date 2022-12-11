use aoc_day_11::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;

fn main() {
    let input = load_text_input_from_autodetect();
    let monkeys = parse_input(&input);

    println!("Part 1: {}", part_one(&monkeys));
    println!("Part 2: {}", part_two(&monkeys));
}
