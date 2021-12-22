use crate::core::{Command, Cuboid, Operation};

pub fn part_two(commands: &[Command]) -> u64 {
    let mut active: Vec<Cuboid> = Vec::with_capacity(commands.len() * 2);
    let mut processed = Vec::with_capacity(commands.len() * 2);

    for cmd in commands.iter() {
        for kube in active.drain(..) {
            kube.collect_non_overlapping(&cmd.cuboid, &mut processed);
        }

        if Operation::On == cmd.op {
            processed.push(cmd.cuboid.clone());
        }

        std::mem::swap(&mut active, &mut processed);
    }

    active.iter().map(|c| c.volume()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);
        assert_eq!(1217140271559773, answer);
    }
}
