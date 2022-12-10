use aoc_shared::input::load_text_input_from_autodetect;

use aoc_day_10::{parse_input, part_one, part_two};

fn main() {
    let input = load_text_input_from_autodetect();
    let instructions = parse_input(&input);

    println!("Part 1: {}", part_one(&instructions));
    let answer = part_two(&instructions);
    for row in answer {
        println!("{}", row.iter().collect::<String>());
    }
}
