use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::Food;

pub fn solve(foods: &[Food]) -> usize {
    let mut ingredients_by_allergen: HashMap<&str, HashSet<&str>> = HashMap::new();

    for food in foods.iter() {
        for allergen in food.allergens.iter().copied() {
            match ingredients_by_allergen.entry(allergen) {
                Entry::Vacant(e) => {
                    e.insert(food.ingredients.clone());
                }
                Entry::Occupied(mut e) => {
                    let ingredients = e.get_mut();
                    ingredients.retain(|i| food.ingredients.contains(i));
                }
            }
        }
    }

    let foods_with_allergens = ingredients_by_allergen
        .values()
        .flatten()
        .copied()
        .collect::<HashSet<_>>();

    foods
        .iter()
        .map(|f| f.ingredients.iter())
        .flatten()
        .copied()
        .filter(|i| !foods_with_allergens.contains(i))
        .count()
}
