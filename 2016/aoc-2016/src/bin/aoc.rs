use std::ops::AddAssign;
use std::time::Duration;

use aoc_2016_common::input::default_input;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 && args.len() != 3 {
        println!("usage: aoc <day> <puzzle_input>");
        return;
    }

    let mut puzzles = [None; 25];
    puzzles[0] = Some(aoc_2016_01::demo);

    let selection = &args[1];
    if selection == "all" {
        let mut runtime = Duration::default();
        for (day, puzzle) in puzzles.iter().enumerate() {
            if let Some(puzzle) = puzzle {
                let elapsed = puzzle(default_input(day + 1));
                runtime.add_assign(elapsed);
            }
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

    match puzzles[day - 1] {
        None => {
            println!("Puzzle {} is not implemented", day);
        }

        Some(puzzle) => {
            let runtime = puzzle(puzzle_input);
            print_total_exec_time(runtime);
        }
    }
}

fn print_total_exec_time(runtime: Duration) {
    println!("---------");
    println!("Total execution time: {:.3?}", runtime);
}
