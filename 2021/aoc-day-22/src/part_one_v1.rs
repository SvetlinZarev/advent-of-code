use crate::core::{Command, Operation};

pub fn part_one_v1(commands: &[Command]) -> usize {
    let mut grid = vec![vec![vec![false; 101]; 101]; 101];

    for cmd in commands {
        let c = &cmd.cuboid;
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

        for x in c.a.0..c.b.0 {
            for y in c.a.1..c.b.1 {
                for z in c.a.2..c.b.2 {
                    let x = (x + 50) as usize;
                    let y = (y + 50) as usize;
                    let z = (z + 50) as usize;

                    grid[x][y][z] = cmd.op == Operation::On;
                }
            }
        }
    }

    grid.iter()
        .flat_map(|y| y.iter().flat_map(|z| z.iter().copied()))
        .filter(|&v| v)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_one_v1() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one_v1(&input);
        assert_eq!(556501, answer);
    }
}
