use aoc_shared::hashing::HashMap;

pub fn part_one<const GREEDY: bool>(graph: &[(u16, Vec<usize>)]) -> u16 {
    assert!(graph.len() <= 64);

    dfs::<GREEDY>(&mut HashMap::default(), graph, 0, 0, 0, 30)
}

fn dfs<const GREEDY: bool>(
    cache: &mut HashMap<(u64, u16, u16), u16>,
    graph: &[(u16, Vec<usize>)],
    current: usize,
    opened: u64,
    rate: u16,
    steps: u16,
) -> u16 {
    if steps == 0 {
        return 0;
    }

    if let Some(&released) = cache.get(&(opened, steps, current as u16)) {
        return released;
    }

    let mut released = 0;

    // Can we open the current valve and does it makes sense to do it ?
    if opened & (1 << current) == 0 && graph[current].0 > 0 {
        let next_opened = opened | 1 << current;
        let next_rate = rate + graph[current].0;
        released = dfs::<GREEDY>(cache, graph, current, next_opened, next_rate, steps - 1);
    } else if GREEDY {
        for next in graph[current].1.iter().copied() {
            let rel = dfs::<GREEDY>(cache, graph, next, opened, rate, steps - 1);
            released = released.max(rel);
        }
    }

    // In the general case this should always be executed. But we can make the solution
    // greedy, which means that it will execute it only in certain cases - i.e. only when we did not open any valve
    if !GREEDY {
        for next in graph[current].1.iter().copied() {
            let rel = dfs::<GREEDY>(cache, graph, next, opened, rate, steps - 1);
            released = released.max(rel);
        }
    }

    released += rate;
    cache.insert((opened, steps, current as u16), released);
    released
}

#[cfg(test)]
mod tests {
    use crate::p1v1::part_one;
    use crate::parse_input;
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let graph = parse_input(input);
        let answer = part_one::<false>(&graph);
        assert_eq!(1584, answer);
    }
}
