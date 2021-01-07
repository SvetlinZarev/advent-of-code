use std::cmp::Ordering;

use aoc_2015_common::ops::{max_by, min_by};

use crate::Routes;

pub fn solve<F>(routes: &Routes, cost_fn: F) -> u64
where
    F: Copy + Fn(u64, u64) -> Ordering,
{
    let mut visited = vec![false; routes.cities];
    tsp(&routes, &mut visited, routes.cities, 0, cost_fn)
}

fn tsp<F>(routes: &Routes, visited: &mut [bool], remaining: usize, from: usize, cost_fn: F) -> u64
where
    F: Copy + Fn(u64, u64) -> Ordering,
{
    if remaining == 1 {
        for i in 0..routes.cities {
            if !visited[i] {
                return routes.distance(from, i);
            }
        }
    }

    let mut cost = max_by(0, u64::max_value(), cost_fn);
    for city in 0..routes.cities {
        if visited[city] || city == from {
            continue;
        }
        visited[city] = true;

        let mut dist = 0;
        if remaining != routes.cities {
            dist = routes.distance(from, city);
        };

        cost = min_by(
            cost,
            dist + tsp(routes, visited, remaining - 1, city, cost_fn),
            cost_fn,
        );

        visited[city] = false;
    }

    cost
}
