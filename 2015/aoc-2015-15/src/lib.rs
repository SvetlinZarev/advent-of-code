use std::ops::{Add, Mul};
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

pub const DAY: usize = 15;

const AMOUNT: u32 = 100;
const P1_CALORIES: Option<u32> = None;
const P2_CALORIES: Option<u32> = Some(500);

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let (dp, ingredients) = measure(DAY, "parsing", || parse_input(&input));
    let (d1, _) = measure(DAY, "part 1", || solve(&ingredients, AMOUNT, P1_CALORIES));
    let (d2, _) = measure(DAY, "part 2", || solve(&ingredients, AMOUNT, P2_CALORIES));

    dp + d1 + d2
}

#[derive(Debug, Copy, Clone)]
struct Components {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}

impl Components {
    pub fn score(self) -> u32 {
        if self.capacity <= 0 || self.durability <= 0 || self.texture <= 0 || self.flavor <= 0 {
            return 0;
        }

        (self.capacity * self.flavor * self.texture * self.durability) as u32
    }
}

impl Default for Components {
    fn default() -> Self {
        Components {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }
}

impl Mul<u32> for Components {
    type Output = Components;

    fn mul(self, rhs: u32) -> Self::Output {
        Components {
            capacity: self.capacity * rhs as i32,
            durability: self.durability * rhs as i32,
            flavor: self.flavor * rhs as i32,
            texture: self.texture * rhs as i32,
            calories: self.calories * rhs,
        }
    }
}

impl Add for Components {
    type Output = Components;

    fn add(self, rhs: Self) -> Self::Output {
        Components {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

fn parse_input(input: &str) -> Vec<Components> {
    let mut ingredients = vec![];

    for line in input.lines() {
        let mut idx = line.find(' ').unwrap();
        let line = &line[idx + 1..];

        let mut ingredient = Components::default();

        for component in line.split(',') {
            let component = component.trim();

            idx = component.find(' ').unwrap();
            let value = component[idx + 1..].parse().unwrap();

            match &component[..idx] {
                "capacity" => ingredient.capacity = value,
                "durability" => ingredient.durability = value,
                "flavor" => ingredient.flavor = value,
                "texture" => ingredient.texture = value,
                "calories" => ingredient.calories = value as u32,
                _ => panic!("Unknown component: {}", component),
            }
        }

        ingredients.push(ingredient);
    }

    ingredients
}

fn solve(ingredients: &[Components], max_amount: u32, calories: Option<u32>) -> u32 {
    // TODO: come up with a generic solution that works with any number of ingredients
    assert_eq!(ingredients.len(), 4);

    let mut score = 0;
    for a in 0..=max_amount {
        for b in 0..=max_amount - a {
            for c in 0..=max_amount - a - b {
                let components = ingredients[0] * a
                    + ingredients[1] * b
                    + ingredients[2] * c
                    + ingredients[3] * (max_amount - a - b - c);

                if let Some(calories) = calories {
                    if components.calories != calories {
                        continue;
                    }
                }

                score = score.max(components.score());
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_input(&input);
        let answer = solve(&parsed, AMOUNT, P1_CALORIES);
        assert_eq!(222870, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let parsed = parse_input(&input);
        let answer = solve(&parsed, AMOUNT, P2_CALORIES);
        assert_eq!(117936, answer);
    }
}
