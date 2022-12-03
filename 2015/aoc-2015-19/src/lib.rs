use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

mod part_one;
mod part_two;

const DAY: usize = 19;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (dp, (molecule, replacements)) = measure(DAY, "parsing", || parse_input(&input));
    let (d1, _) = measure(DAY, "part 1", || solve_part_one(&molecule, &replacements));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(&molecule, &replacements));

    dp + d1 + d2
}

fn parse_input(input: &str) -> (String, HashMap<String, Vec<String>>) {
    let mut parse_replacements = true;
    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();
    let mut molecule = String::new();

    for line in input.lines() {
        if line.is_empty() {
            if parse_replacements {
                parse_replacements = false;
                continue;
            }

            panic!("Unexpected new line");
        }

        if parse_replacements {
            let idx = line.find(' ').unwrap();
            let from = line[..idx].to_owned();
            let to = line[idx + 4..].to_owned();

            match replacements.entry(from) {
                Entry::Occupied(mut e) => {
                    let entry = e.get_mut();
                    entry.push(to);
                }

                Entry::Vacant(e) => {
                    let entry = vec![to];
                    e.insert(entry);
                }
            }
        } else {
            if !(molecule.is_empty()) {
                panic!("The molecule was already read");
            }

            molecule.push_str(line);
        }
    }

    (molecule, replacements)
}

fn solve_part_one(molecule: &str, replacements: &HashMap<String, Vec<String>>) -> usize {
    part_one::solve(molecule, replacements)
}

fn solve_part_two(molecule: &str, replacements: &HashMap<String, Vec<String>>) -> u32 {
    part_two::solve(molecule, replacements)
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let (molecule, replacements) = parse_input(&input);

        let answer = solve_part_one(&molecule, &replacements);
        assert_eq!(509, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let (molecule, replacements) = parse_input(&input);

        let answer = solve_part_two(&molecule, &replacements);
        assert_eq!(195, answer);
    }
}
