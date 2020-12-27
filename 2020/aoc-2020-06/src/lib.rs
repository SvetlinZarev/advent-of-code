use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 6;

const MASK_SET_NEW_LINE: u32 = 1 << 31;
const MASK_CLEAR_NEW_LINE: u32 = u32::max_value() >> 1;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    measure(DAY, "part 1: iterators", || {
        part_one::solve_iter(input.as_str())
    });
    let (d_1, _) = measure(DAY, "part 1: loops", || {
        part_one::solve_iter(input.as_str())
    });

    measure(DAY, "part 2: iterators", || {
        part_two::solve_iter(input.as_str())
    });
    let (d_2, _) = measure(DAY, "part 2: loops", || {
        part_two::solve_iter(input.as_str())
    });

    d_1.add(d_2)
}

fn solve_iter(input: &str, initial: u32, fold_function: fn(u32, u32) -> u32) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|l| l.as_bytes().iter())
                .map(|b| b.fold(0u32, |acc, &x| acc | (1 << (x as u32 - b'a' as u32))))
                .fold(initial, fold_function)
                .count_ones()
        })
        .sum()
}

#[inline(never)]
fn solve_loop(input: &[u8], init: u32, fold_function: fn(a: u32, b: u32) -> u32) -> u32 {
    let mut solution = 0u32;

    let mut group_answers = init;
    let mut person_answers = 0u32;

    for &x in input {
        match x {
            b'\n' if group_answers & MASK_SET_NEW_LINE == MASK_SET_NEW_LINE => {
                // This is the second time we've encountered a new line,
                // so update the solution and reset the internal state
                solution += group_answers.count_ones() - 1;

                //reset state
                group_answers = init;
                person_answers = 0;
            }

            b'\n' => {
                // this will also clean the new line flag,
                // so we'll have to add it again
                group_answers = fold_function(group_answers, person_answers);
                group_answers |= MASK_SET_NEW_LINE;
                person_answers = 0;
            }

            b'a'..=b'z' => {
                let shift = x as u32 - b'a' as u32;
                person_answers |= 1 << shift;
                group_answers &= MASK_CLEAR_NEW_LINE;
            }

            _ => unreachable!(),
        }
    }

    // Handle any leftovers due to only zero/one new-line at the end of the file
    // ---
    // There are three cases:
    // 1.) Everything is handled:
    //     - in this case person_answers == 0 && group_answers == init
    // 2.) Person's answers were committed, but group_answers were not
    //     - in this case person_answers == 0 and group_answers != init
    // 3.) Both person_answers and group_answers were not committed
    //     - in this case person_answers != 0 and group_answers != init
    //
    // In order to handle all three cases we can clear the new-line mask,
    // which is always safe to do. Then we must check if person_answers was
    // committed, because the fold_function might have destructive effect
    // when person_data == 0. Then we update the solution with the leftovers

    group_answers &= MASK_CLEAR_NEW_LINE;
    if person_answers != 0 {
        group_answers = fold_function(group_answers, person_answers);
    }
    solution + group_answers.count_ones()
}

#[cfg(test)]
mod tests {
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));

        let solution = part_one::solve_iter(input.as_str());
        assert_eq!(6443, solution);

        let solution = part_one::solve_loops(input.as_bytes());
        assert_eq!(6443, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));

        let solution = part_two::solve_iter(input.as_str());
        assert_eq!(3232, solution);

        let solution = part_two::solve_loops(input.as_bytes());
        assert_eq!(3232, solution);
    }
}
