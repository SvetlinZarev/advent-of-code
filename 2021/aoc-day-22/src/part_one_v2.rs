use crate::core::{Command, Cuboid, Operation};

pub fn part_one_v2(commands: &[Command]) -> u64 {
    let mut active: Vec<Cuboid> = Vec::with_capacity(commands.len() * 2);
    let mut processed = Vec::with_capacity(commands.len() * 2);

    for cmd in commands.iter() {
        let c = cmd.cuboid.clone();
        if c.a.0 < -50 || c.a.0 > 50 {
            continue;
        }
        if c.a.1 < -50 || c.a.1 > 50 {
            continue;
        }
        if c.a.2 < -50 || c.a.2 > 50 {
            continue;
        }
        if c.b.0 < -50 || c.b.0 > 50 {
            continue;
        }
        if c.b.1 < -50 || c.b.1 > 50 {
            continue;
        }
        if c.b.2 < -50 || c.b.2 > 50 {
            continue;
        }

        for kube in active.drain(..) {
            if let Some(intersection) = kube.intersect(&c) {
                let intersections = kube.subtract(&intersection);
                processed.extend(intersections);
            } else {
                processed.push(kube);
            }
        }

        if Operation::On == cmd.op {
            processed.push(c);
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
        let answer = part_one_v2(&input);
        assert_eq!(556501, answer);
    }
}
