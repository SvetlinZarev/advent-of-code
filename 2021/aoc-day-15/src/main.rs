use aoc_day_15::astar_pf::a_star_pf;
use aoc_day_15::{expand_grid, part_one, part_two};
use aoc_shared::input::load_text_input_from_autodetect;
use aoc_shared::parsing::parse_numeric_grid;

fn main() {
    let input = parse_numeric_grid(load_text_input_from_autodetect());
    println!("Part 1: {}", part_one(&input));
    println!("Part 1(pf): {}", a_star_pf(&input));

    let input = expand_grid(&input);
    println!("Part 2: {}", part_two(&input));
    println!("Part 3(pf): {}", a_star_pf(&input));
}
