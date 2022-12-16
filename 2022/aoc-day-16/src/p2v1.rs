use aoc_shared::hashing::HashMap;

// NOTE: Very slow and consumes a lot of memory!
pub fn part_two(graph: &[(u16, Vec<usize>)]) -> u16 {
    assert!(graph.len() <= 64);
    dfs(&mut HashMap::default(), graph, 0, 0, 0, 0, 26)
}

fn dfs(
    // Use u16 for the indexes, because our array is limited to 64 elements due to the bitset!
    cache: &mut HashMap<(u64, u16, u16, u16), u16>,
    graph: &[(u16, Vec<usize>)],
    me: usize,
    el: usize,
    opened: u64,
    rate: u16,
    steps: u16,
) -> u16 {
    if steps == 0 {
        return 0;
    }

    if let Some(&released) = cache.get(&(opened, steps, me as u16, el as u16)) {
        return released;
    }

    let mut released = 0;

    // If we stay at the same position, It's always me that will open the valve,
    // and the elephant will go ahead, because the two cases are equivalent
    let me_open = opened & (1 << me) == 0 && graph[me].0 > 0;
    let el_open = opened & (1 << el) == 0 && graph[el].0 > 0 && me != el;

    match (me_open, el_open) {
        // We'll both open our valves
        (true, true) => {
            let next_opened = opened | (1 << me) | (1 << el);
            let next_rate = rate + graph[me].0 + graph[el].0;
            let rel = dfs(cache, graph, me, el, next_opened, next_rate, steps - 1);
            released = released.max(rel);
        }

        // I'll open my valve
        (true, false) => {
            let next_opened = opened | 1 << me;
            let next_rate = rate + graph[me].0;

            for next_el in graph[el].1.iter().copied() {
                let rel = dfs(cache, graph, me, next_el, next_opened, next_rate, steps - 1);
                released = released.max(rel);
            }
        }

        // The elephant will open its valve
        (false, true) => {
            let next_opened = opened | 1 << el;
            let next_rate = rate + graph[el].0;

            for next_me in graph[me].1.iter().copied() {
                let rel = dfs(cache, graph, next_me, el, next_opened, next_rate, steps - 1);
                released = released.max(rel);
            }
        }

        // I'm not sure if that is correct in the general case. This loop
        // in the general case needs to be outside match and be executed
        // on each recursive call
        (false, false) => {
            // We'll both move without opening any valves
            for next_me in graph[me].1.iter().copied() {
                for next_el in graph[el].1.iter().copied() {
                    let rel = dfs(cache, graph, next_me, next_el, opened, rate, steps - 1);
                    released = released.max(rel);
                }
            }
        }
    }

    released += rate;
    cache.insert((opened, steps, me as u16, el as u16), released);
    released
}
