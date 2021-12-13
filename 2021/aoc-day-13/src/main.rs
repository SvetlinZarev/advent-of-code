use aoc_day_12::{parse_input, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;

fn main() {
    let (points, fold_instr) = parse_input(load_text_input_from_autodetect());

    println!("Part 1: {}", part_one(&points, &fold_instr));
    println!("Part 2:");
    let grid = part_two(&points, &fold_instr);
    for row in grid {
        let string = row.iter().collect::<String>();
        println!("{}", string);
    }
}
