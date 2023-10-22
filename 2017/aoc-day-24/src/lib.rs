use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Port {
    a: u32,
    b: u32,
}

impl FromStr for Port {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((a, b)) = s.split_once('/') else {
            return Err(format!("invalid port definition: {}", s).into());
        };

        Ok(Port {
            a: a.parse()?,
            b: b.parse()?,
        })
    }
}

pub fn part_one(input: &[Port]) -> u32 {
    let mut by_port_size = HashMap::new();
    for (idx, port) in input.iter().copied().enumerate() {
        by_port_size.entry(port.a).or_insert(vec![]).push(idx);
        by_port_size.entry(port.b).or_insert(vec![]).push(idx);
    }

    let mut used = vec![false; input.len()];
    dfs_part_1(&mut used, input, &by_port_size, 0)
}

fn dfs_part_1(
    used: &mut [bool],
    ports: &[Port],
    by_port_size: &HashMap<u32, Vec<usize>>,
    port_size: u32,
) -> u32 {
    let mut answer = 0;

    if let Some(port_indexes) = by_port_size.get(&port_size) {
        for port_idx in port_indexes.iter().copied() {
            if used[port_idx] {
                continue;
            }

            used[port_idx] = true;

            let port = ports[port_idx];
            let next_port_size = if port.a == port_size { port.b } else { port.a };

            let mut val = ports[port_idx].a + ports[port_idx].b;
            val += dfs_part_1(used, ports, by_port_size, next_port_size);
            answer = answer.max(val);

            used[port_idx] = false;
        }
    }

    answer
}

pub fn part_two(input: &[Port]) -> u32 {
    let mut by_port_size = HashMap::new();
    for (idx, port) in input.iter().copied().enumerate() {
        by_port_size.entry(port.a).or_insert(vec![]).push(idx);
        by_port_size.entry(port.b).or_insert(vec![]).push(idx);
    }

    let mut used = vec![false; input.len()];
    dfs_part_2(&mut used, input, &by_port_size, 0).1
}

fn dfs_part_2(
    used: &mut [bool],
    ports: &[Port],
    by_port_size: &HashMap<u32, Vec<usize>>,
    port_size: u32,
) -> (u32, u32) {
    let mut best_len = 0;
    let mut best_str = 0;

    if let Some(port_indexes) = by_port_size.get(&port_size) {
        for port_idx in port_indexes.iter().copied() {
            if used[port_idx] {
                continue;
            }

            used[port_idx] = true;

            let port = ports[port_idx];
            let next_port_size = if port.a == port_size { port.b } else { port.a };
            let add_str = ports[port_idx].a + ports[port_idx].b;

            let (length, strength) = dfs_part_2(used, ports, by_port_size, next_port_size);
            if length + 1 > best_len {
                best_len = length + 1;
                best_str = strength + add_str;
            } else if length + 1 == best_len {
                best_str = best_str.max(strength + add_str);
            }

            used[port_idx] = false;
        }
    }

    (best_len, best_str)
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(1695, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(1673, answer);
    }
}
