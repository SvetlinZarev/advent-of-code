use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::Food;

pub fn solve(foods: &[Food]) -> String {
    let mut ingredients_by_allergen = HashMap::<&str, HashSet<&str>>::new();

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

    // the btree map is sorted by definition
    let mut mapped_allergens = BTreeMap::new();
    while !ingredients_by_allergen.is_empty() {
        // Store the allergens that have a 1:1 relationship with an ingredient
        ingredients_by_allergen
            .iter()
            .filter(|(_, v)| v.len() == 1)
            .for_each(|(&k, v)| {
                mapped_allergens.insert(k, *v.iter().next().unwrap());
            });

        // remove the 1:1 mapped allergens
        for mapped_allergen in mapped_allergens.keys() {
            ingredients_by_allergen.remove(mapped_allergen);
        }

        // remove the 1:1 mapped ingredients from the ingredient lists of the remaining allergens
        ingredients_by_allergen
            .values_mut()
            .for_each(|ingredients| {
                for ing in mapped_allergens.values() {
                    ingredients.remove(ing);
                }
            });
    }

    mapped_allergens
        .values()
        .copied()
        .fold(String::with_capacity(128), |mut acc, v| {
            if !acc.is_empty() {
                acc.push(',');
            }
            acc.push_str(v);
            acc
        })
}
