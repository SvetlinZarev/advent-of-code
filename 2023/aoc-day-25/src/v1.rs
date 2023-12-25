use rand::prelude::*;

use crate::{count_reachable, parse_input};

pub fn part_one(input: &str) -> usize {
    let (mut graph, mut connections) = parse_input(input);

    // protect against malicious input
    connections.shuffle(&mut thread_rng());

    // Select 2 edges to remove and then run Tarjan's bridge finding
    // algorithm to find the third.
    for i in 0..connections.len() - 1 {
        println!("Progress: {}/{}", i + 1, connections.len());

        // remove the first connection
        let (a, b) = connections[i];
        graph[a].retain(|&x| x != b);
        graph[b].retain(|&x| x != a);

        for j in i + 1..connections.len() {
            // remove the second connection
            let (p, q) = connections[j];
            graph[p].retain(|&x| x != q);
            graph[q].retain(|&x| x != p);

            // If we find a third bridge, then we know the first and second edges we removed are correct
            if let Some((m, n)) = critical_connection(&graph) {
                // remove the third connection
                graph[m].retain(|&x| x != n);
                graph[n].retain(|&x| x != m);

                let set_a = count_reachable(&graph, 0);
                let set_b = graph.len() - set_a;
                return set_a * set_b;
            }

            // restore the second connection
            graph[p].push(q);
            graph[q].push(p);
        }

        // restore the first connection
        graph[a].push(b);
        graph[b].push(a);
    }

    panic!("failed to find solution")
}

pub fn critical_connection(graph: &[Vec<usize>]) -> Option<(usize, usize)> {
    let mut discovery = vec![0; graph.len()];
    let mut earliest = vec![0; graph.len()];

    dfs(&graph, &mut discovery, &mut earliest, 1, graph.len(), 0)
}

fn dfs(
    graph: &[Vec<usize>],
    discovery: &mut [usize],
    earliest: &mut [usize],
    counter: usize,
    parent: usize,
    node: usize,
) -> Option<(usize, usize)> {
    // Add a guard to avoid visiting a node for a second time
    // if discovery[node] != 0 {
    //     return;
    // }

    discovery[node] = counter;
    earliest[node] = counter;

    for &child in graph[node].iter() {
        if child == parent {
            continue;
        }

        // a discovery time of 0, means that the node has not been visited yet
        if discovery[child] == 0 {
            let bridge = dfs(graph, discovery, earliest, counter + 1, node, child);
            if bridge.is_some() {
                return bridge;
            }

            earliest[node] = earliest[node].min(earliest[child]);
            if discovery[node] < earliest[child] {
                return Some((node, child));
            }
        } else {
            earliest[node] = earliest[node].min(discovery[child]);
        }
    }

    None
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
