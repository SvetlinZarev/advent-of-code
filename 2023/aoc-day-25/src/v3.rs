use aoc_shared::algo::UnionFind;
use rand::prelude::*;
use rand::thread_rng;

use crate::parse_input;

const EXPECTED_CUTS: i32 = 3;

pub fn part_one(input: &str) -> usize {
    let (graph, mut connections) = parse_input(input);
    let mut uf = UnionFind::new(graph.len());

    loop {
        // shuffle in order to be able to easily choose a "random" edge
        connections.shuffle(&mut thread_rng());
        let mut edges = connections.as_slice();
        uf.reset();

        while uf.number_of_groups() > 2 {
            let (a, b) = edges[0];
            edges = &edges[1..];

            uf.union(a, b);
        }

        let mut cuts = 0;
        for &(a, b) in edges.iter() {
            if uf.find(a) != uf.find(b) {
                cuts += 1;
            }
        }

        if cuts == EXPECTED_CUTS {
            let root = uf.find(0);
            let size = uf.group_size(root);

            return (graph.len() - size) * size;
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");

        let answer = part_one(&input);
        assert_eq!(580_800, answer);
    }
}
