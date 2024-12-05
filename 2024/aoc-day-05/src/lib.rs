use std::cmp::Ordering;
use std::collections::VecDeque;
use std::error::Error;

pub fn parse_input(input: &str) -> Result<(Vec<u128>, Vec<Vec<usize>>), Box<dyn Error>> {
    let mut graph = Vec::with_capacity(100);
    let mut updates = Vec::with_capacity(200);

    let mut parsing_graph = true;
    for line in input.lines() {
        if line.is_empty() {
            parsing_graph = false;
            continue;
        }

        match parsing_graph {
            true => {
                let line = line.as_bytes();
                let a = (line[0] - b'0') as usize * 10 + (line[1] - b'0') as usize;
                let b = (line[3] - b'0') as usize * 10 + (line[4] - b'0') as usize;

                let max = a.max(b);
                if graph.len() <= max {
                    graph.resize(max + 1, 0u128);
                }

                graph[a] = graph[a] | (1 << b);
            }
            false => {
                let update = line
                    .as_bytes()
                    .chunks(3)
                    .map(|x| (x[0] - b'0') as usize * 10 + (x[1] - b'0') as usize)
                    .collect::<Vec<_>>();
                updates.push(update);
            }
        }
    }

    Ok((graph, updates))
}

pub fn part_one(graph: &Vec<u128>, updates: &Vec<Vec<usize>>) -> u64 {
    let mut answer = 0;

    for update in updates {
        if is_ordered(&graph, update) {
            answer += update[update.len() / 2] as u64;
        }
    }

    answer
}

// This works on the assumption that the rules define a total order, which might not be the case.
// it works on this input, but it might not work for any input.
// Let's have "A before B" and "B before C". This does nto mean we have A before C
pub fn part_two_sorting(graph: &Vec<u128>, updates: &Vec<Vec<usize>>) -> u64 {
    let mut answer = 0;

    let mut buffer = vec![];
    for update in updates {
        if !is_ordered(&graph, update) {
            buffer.clear();
            buffer.extend_from_slice(&update);

            let (_, val, _) = buffer.select_nth_unstable_by(update.len() / 2, |&a, b| {
                if graph[a] & (1u128 << b) != 0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            answer += *val as u64;
        }
    }

    answer
}

pub fn part_two_topo_sort(graph: &Vec<u128>, updates: &Vec<Vec<usize>>) -> u64 {
    let mut sum = 0;
    let mut queue = VecDeque::new();

    for update in updates {
        if !is_ordered(&graph, update) {
            let mut pages = 0u128;
            for &page in update {
                pages |= 1 << page;
            }

            let mut indegree = [0u32; 100];
            for (node, mut dependants) in graph.iter().copied().enumerate() {
                if pages & (1 << node) != 0 {
                    while dependants != 0 {
                        let next_page = dependants.trailing_zeros() as usize;
                        dependants &= dependants - 1;

                        if pages & (1 << next_page) != 0 {
                            indegree[next_page] += 1;
                        }
                    }
                }
            }

            for idx in 0..indegree.len() {
                if indegree[idx] == 0 && pages & (1 << idx) != 0 {
                    queue.push_back(idx);
                }
            }

            let mut k = update.len() / 2;
            while let Some(node) = queue.pop_front() {
                if k == 0 {
                    sum += node;
                    break;
                }
                k -= 1;

                let mut next = graph[node];
                while next != 0 {
                    let id = next.trailing_zeros() as usize;
                    next &= next - 1;

                    if pages & (1 << id) != 0 {
                        indegree[id] -= 1;
                        if indegree[id] == 0 {
                            queue.push_back(id);
                        }
                    }
                }
            }
        }
    }

    sum as u64
}

fn is_ordered(ordering: &[u128], update: &[usize]) -> bool {
    update.len() <= 1
        || update.windows(2).all(|w| {
            let next = ordering[w[0]];
            next & (1u128 << w[1]) != 0
        })
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (graph, updates) = parse_input(&input).unwrap();

        let answer = part_one(&graph, &updates);
        assert_eq!(5166, answer);
    }

    #[test]
    fn test_part_two_sorting() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (graph, updates) = parse_input(&input).unwrap();

        let answer = part_two_sorting(&graph, &updates);
        assert_eq!(4679, answer);
    }

    #[test]
    fn test_part_two_topo_sort() {
        let input = load_text_input_from_file("inputs/input.txt");
        let (graph, updates) = parse_input(&input).unwrap();

        let answer = part_two_topo_sort(&graph, &updates);
        assert_eq!(4679, answer);
    }
}
