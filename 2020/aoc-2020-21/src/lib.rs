use aoc_2020_common::input::load_input;
use aoc_2020_common::output::measure_solution;
use std::collections::HashSet;
use std::path::Path;

pub mod part_one;
pub mod part_two;

pub const DEFAULT_INPUT_PATH: &str = "../puzzle-inputs/day-21.txt";

pub fn demo<P: AsRef<Path>>(path: P) {
    let input = load_input(path);
    let foods = parse_input(&input);

    measure_solution(21, 1, "", || part_one::solve(&foods));
    measure_solution(21, 2, "", || part_two::solve(&foods));
}

#[derive(Debug, Clone)]
pub struct Food<'a> {
    pub ingredients: HashSet<&'a str>,
    pub allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    pub fn new(ingredients: HashSet<&'a str>, allergens: HashSet<&'a str>) -> Food<'a> {
        Food {
            ingredients,
            allergens,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Food> {
    let mut foods = vec![];

    for line in input.lines() {
        let ingredients_end = match line.rfind('(') {
            None => line.len(),
            Some(idx) => idx - 1, // -1 because of the space ' ' before the bracket
        };

        let ingredients = line[..ingredients_end].split(' ').collect();

        let mut allergens = HashSet::new();
        if ingredients_end != line.len() {
            allergens = line[ingredients_end + 11..line.len() - 1]
                .split(',')
                .map(|v| v.trim())
                .collect();
        }

        let food = Food::new(ingredients, allergens);
        foods.push(food);
    }

    foods
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let foods = parse_input(&input);

        let solution = part_one::solve(&foods);
        assert_eq!(2324, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(DEFAULT_INPUT_PATH);
        let foods = parse_input(&input);

        let solution = part_two::solve(&foods);
        assert_eq!("bxjvzk,hqgqj,sp,spl,hsksz,qzzzf,fmpgn,tpnnkc", solution);
    }
}
