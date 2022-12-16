use aoc_shared::hashing::HashMap;

// NOTE: Very slow and consumes a lot of memory!
pub fn part_two<const GREEDY: bool>(graph: &[(u16, Vec<usize>)]) -> u16 {
    assert!(graph.len() <= 64);
    dfs::<GREEDY>(&mut HashMap::default(), graph, 0, 0, 0, 0, 26)
}

fn dfs<const GREEDY: bool>(
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
        (true, true) => {
            // We'll bot open (different) valves
            {
                let next_opened = opened | (1 << me) | (1 << el);
                let next_rate = rate + graph[me].0 + graph[el].0;

                let rel = dfs::<GREEDY>(cache, graph, me, el, next_opened, next_rate, steps - 1);
                released = released.max(rel);
            }

            if !GREEDY {
                //I'll open and the elephant will not
                {
                    let next_opened = opened | 1 << me;
                    let next_rate = rate + graph[me].0;

                    for next_el in graph[el].1.iter().copied() {
                        let rel = dfs::<GREEDY>(
                            cache,
                            graph,
                            me,
                            next_el,
                            next_opened,
                            next_rate,
                            steps - 1,
                        );
                        released = released.max(rel);
                    }
                }

                // The elephant will open and I'll not
                let next_opened = opened | 1 << el;
                let next_rate = rate + graph[el].0;

                for next_me in graph[me].1.iter().copied() {
                    let rel =
                        dfs::<GREEDY>(cache, graph, next_me, el, next_opened, next_rate, steps - 1);
                    released = released.max(rel);
                }
            }
        }

        // I'll open the valve, or ir if we stand at the sme valve, then I'll open it
        (true, false) => {
            let next_opened = opened | 1 << me;
            let next_rate = rate + graph[me].0;

            for next_el in graph[el].1.iter().copied() {
                let rel =
                    dfs::<GREEDY>(cache, graph, me, next_el, next_opened, next_rate, steps - 1);
                released = released.max(rel);
            }
        }

        // The elephant will open its valve
        (false, true) => {
            let next_opened = opened | 1 << el;
            let next_rate = rate + graph[el].0;

            for next_me in graph[me].1.iter().copied() {
                let rel =
                    dfs::<GREEDY>(cache, graph, next_me, el, next_opened, next_rate, steps - 1);
                released = released.max(rel);
            }
        }

        // This loop in the general case needs to be outside match and be executed on each recursive call
        (false, false) => {
            if GREEDY {
                // We'll both move without opening any valves
                for next_me in graph[me].1.iter().copied() {
                    for next_el in graph[el].1.iter().copied() {
                        let rel =
                            dfs::<GREEDY>(cache, graph, next_me, next_el, opened, rate, steps - 1);
                        released = released.max(rel);
                    }
                }
            }
        }
    }

    // In the general case this should always be executed. But we can make the solution
    // greedy, which means that it will execute it only in certain cases - i.e. only when we did not open any valve
    if !GREEDY {
        // We'll both move without opening any valves
        for next_me in graph[me].1.iter().copied() {
            for next_el in graph[el].1.iter().copied() {
                let rel = dfs::<GREEDY>(cache, graph, next_me, next_el, opened, rate, steps - 1);
                released = released.max(rel);
            }
        }
    }

    released += rate;
    cache.insert((opened, steps, me as u16, el as u16), released);
    released
}
