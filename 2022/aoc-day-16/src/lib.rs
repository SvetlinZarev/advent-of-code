use aoc_shared::hashing::HashMap;

pub mod p1v1;
pub mod p2v1;

pub mod p1v2;
pub mod p2v2;

pub fn parse_input(input: impl AsRef<str>) -> Vec<(u16, Vec<usize>)> {
    let input = input.as_ref();

    let mut graph = vec![(0, vec![]); 1];
    let mut valves = HashMap::default();
    valves.insert("AA", 0);

    for line in input.lines() {
        let (valve, rest) = line[6..].split_once(' ').unwrap();
        let idx = *valves.entry(valve).or_insert(graph.len());

        let (rate, rest) = rest[14..].split_once(';').unwrap();
        let rate = rate.parse().unwrap();

        if graph.len() <= idx {
            graph.push((0, vec![]));
        }
        graph[idx].0 = rate;

        let rest = &rest[21..];
        let (_, rest) = rest.split_once(' ').unwrap();

        for v in rest.split(", ") {
            let next = *valves.entry(v).or_insert(graph.len());
            if graph.len() <= next {
                graph.push((0, vec![]));
            }

            graph[idx].1.push(next);
        }
    }

    graph
}

// Note: This seems similar to the "Okabe and city" problem. Maybe we can
// transform the graph to another wighted one containing only nodes with
// flow rate > 0
