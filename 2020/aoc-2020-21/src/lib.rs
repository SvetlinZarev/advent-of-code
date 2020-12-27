use std::collections::HashSet;
use std::ops::Add;
use std::path::Path;
use std::time::Duration;

use aoc_2020_common::input::load_input;
use aoc_2020_common::timing::measure;

pub mod part_one;
pub mod part_two;

pub const DAY: usize = 21;

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);

    let (dp, foods) = measure(DAY, "parsing", || parse_input(&input));
    let (d1, _) = measure(DAY, "part 1", || part_one::solve(&foods));
    let (d2, _) = measure(DAY, "part 2", || part_two::solve(&foods));

    dp.add(d1).add(d2)
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
    use aoc_2020_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let foods = parse_input(&input);

        let solution = part_one::solve(&foods);
        assert_eq!(2324, solution);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let foods = parse_input(&input);

        let solution = part_two::solve(&foods);
        assert_eq!("bxjvzk,hqgqj,sp,spl,hsksz,qzzzf,fmpgn,tpnnkc", solution);
    }
}
