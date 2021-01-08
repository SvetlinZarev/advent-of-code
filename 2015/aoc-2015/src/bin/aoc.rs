use std::ops::AddAssign;
use std::time::Duration;

use aoc_2015_common::input::default_input;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 && args.len() != 3 {
        println!("usage: aoc <day> <puzzle_input>");
        return;
    }

    let puzzles = [
        aoc_2015_01::demo,
        aoc_2015_02::demo,
        aoc_2015_03::demo,
        aoc_2015_04::demo,
        aoc_2015_05::demo,
        aoc_2015_06::demo,
        aoc_2015_07::demo,
        aoc_2015_08::demo,
        aoc_2015_09::demo,
        aoc_2015_10::demo,
        aoc_2015_11::demo,
        aoc_2015_12::demo,
        aoc_2015_13::demo,
        aoc_2015_14::demo,
        aoc_2015_15::demo,
        aoc_2015_16::demo,
        aoc_2015_17::demo,
        aoc_2015_18::demo,
        aoc_2015_19::demo,
        aoc_2015_20::demo,
        aoc_2015_21::demo,
        aoc_2015_22::demo,
    ];

    let selection = &args[1];
    if selection == "all" {
        let mut runtime = Duration::default();
        for (day, puzzle) in puzzles.iter().enumerate() {
            let elapsed = puzzle(default_input(day + 1));
            runtime.add_assign(elapsed);
        }
        print_total_exec_time(runtime);

        return;
    }

    let day = selection.parse::<usize>().unwrap();
    if day < 1 || day > puzzles.len() {
        println!(
            "Invalid selection. The day must be between 1 and {}",
            puzzles.len()
        );
        return;
    }

    let mut puzzle_input = default_input(day);
    if args.len() == 3 {
        puzzle_input = args[2].to_owned();
    }

    let runtime = puzzles[day - 1](puzzle_input);
    print_total_exec_time(runtime);
}

fn print_total_exec_time(runtime: Duration) {
    println!("---------");
    println!("Total execution time: {:.3?}", runtime);
}
