use std::cmp::Ordering;

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

fn min_by<T, F>(a: T, b: T, comparator: F) -> T
where
    T: Ord + Copy,
    F: Copy + Fn(T, T) -> Ordering,
{
    match comparator(a, b) {
        Ordering::Less => a,
        Ordering::Equal => a,
        Ordering::Greater => b,
    }
}

fn max_by<T, F>(a: T, b: T, comparator: F) -> T
where
    T: Ord + Copy,
    F: Copy + Fn(T, T) -> Ordering,
{
    match comparator(a, b) {
        Ordering::Less => b,
        Ordering::Equal => a,
        Ordering::Greater => a,
    }
}

#[cfg(test)]
mod tests {
    use crate::exhaustive::{max_by, min_by};

    #[test]
    fn test_min_by() {
        let a = 0;
        let b = u32::max_value();

        let x = min_by(a, b, |a, b| a.cmp(&b));
        assert_eq!(a, x);
    }

    #[test]
    fn test_max_by() {
        let a = 0;
        let b = u32::max_value();

        let x = max_by(a, b, |a, b| a.cmp(&b));
        assert_eq!(b, x);
    }
}
