use std::time::Duration;

use aoc_2019_common::input::default_input;
use std::collections::HashMap;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 && args.len() != 3 {
        println!("usage: aoc <day> <puzzle_input>");
        return;
    }

    let mut puzzles = HashMap::new();
    puzzles.insert(aoc_2019_01::DAY, aoc_2019_01::demo);

    let selection = &args[1];
    if selection == "all" {
        let runtime = puzzles
            .iter()
            .map(|(&day, puzzle)| puzzle(default_input(day)))
            .sum();

        print_total_exec_time(runtime);
        return;
    }

    let day = selection.parse::<usize>().unwrap();
    if day < 1 || day > 25 {
        println!("Invalid selection. The day must be between 1 and 25");
        return;
    }

    let mut puzzle_input = default_input(day);
    if args.len() == 3 {
        puzzle_input = args[2].to_owned();
    }

    let runtime = puzzles[&day](puzzle_input);
    print_total_exec_time(runtime);
}

fn print_total_exec_time(runtime: Duration) {
    println!("---------");
    println!("Total execution time: {:.3?}", runtime);
}
