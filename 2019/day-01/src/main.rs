use aoc_shared_2019::input::load_line_delimited_input_from_autodetect;
use day_01::{part_one, part_two};

fn main() {
    let input = load_line_delimited_input_from_autodetect();
    println!("Part 1: {}", part_one::solve(&input));
    println!("Part 2: {}", part_two::solve(&input));
}
