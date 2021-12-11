use crate::fnvhash::{FnvHasher, HashBuilder};
use crate::Line;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn part_two_v1(input: &[Line]) -> usize {
    let mut field = HashMap::with_hasher(HashBuilder::<FnvHasher>::default());

    input.iter().for_each(|l| {
        let (mut x, mut y) = (l.a.x, l.a.y);

        for _ in 0..=l.steps() {
            field.entry((x, y)).and_modify(|v| *v += 1).or_insert(1u32);

            x += (x < l.b.x) as u16;
            x -= (x > l.b.x) as u16;

            y += (y < l.b.y) as u16;
            y -= (y > l.b.y) as u16;
        }
    });

    field.values().filter(|&&v| v > 1).count()
}

pub fn part_two_v2(input: &[Line]) -> usize {
    let mut field = vec![0u8; 1000 * 1000];
    let mut intersections = 0;

    for line in input.iter().copied() {
        if line.is_horizontal() {
            for x in line.a.x.min(line.b.x)..=line.a.x.max(line.b.x) {
                let idx = x as usize * 1000 + line.a.y as usize;
                field[idx] = field[idx].saturating_add(1);
                intersections += (field[idx] == 2) as usize;
            }
        } else if line.is_vertical() {
            for y in line.a.y.min(line.b.y)..=line.a.y.max(line.b.y) {
                let idx = line.a.x as usize * 1000 + y as usize;
                field[idx] = field[idx].saturating_add(1);
                intersections += (field[idx] == 2) as usize;
            }
        } else {
            // This would work for horizontal/vertical lines as well,
            // but using the specialized code yields a 10% runtime reduction
            let (mut x, mut y) = (line.a.x, line.a.y);

            let dx = match line.a.x.cmp(&line.b.x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => u16::MAX,
            };

            let dy = match line.a.y.cmp(&line.b.y) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => u16::MAX,
            };

            loop {
                let idx = x as usize * 1000 + y as usize;
                field[idx] = field[idx].saturating_add(1);
                intersections += (field[idx] == 2) as usize;

                if x == line.b.x && y == line.b.y {
                    break;
                }

                x = x.wrapping_add(dx);
                y = y.wrapping_add(dy);
            }
        }
    }

    // counting the intersections inline instead of
    // at the end yielded a 30% performance increase
    intersections
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_line_delimited_input_from_file;

    #[test]
    fn test_part_two_v1() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two_v1(&input);
        assert_eq!(20196, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two_v2(&input);
        assert_eq!(20196, answer);
    }
}
