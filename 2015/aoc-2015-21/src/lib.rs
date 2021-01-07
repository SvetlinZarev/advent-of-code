use std::cmp::Ordering;
use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::ops::{max_by, min_by};
use aoc_2015_common::timing::measure;

const DAY: usize = 21;
const INITIAL_HP: u32 = 100;

const WEAPONS: &[Item] = &[
    Item::new(8, 4, 0),
    Item::new(10, 5, 0),
    Item::new(25, 6, 0),
    Item::new(40, 7, 0),
    Item::new(74, 8, 0),
];

const ARMOR: &[Item] = &[
    // The first armor is dummy - i.e. - no armor
    Item::new(0, 0, 0),
    Item::new(13, 0, 1),
    Item::new(31, 0, 2),
    Item::new(53, 0, 3),
    Item::new(75, 0, 4),
    Item::new(102, 0, 5),
];

const RINGS: &[Item] = &[
    // The first ring is dummy - i.e. - no ring
    Item::new(0, 0, 0),
    Item::new(20, 0, 1),
    Item::new(25, 1, 0),
    Item::new(40, 0, 2),
    Item::new(80, 0, 3),
    Item::new(50, 2, 0),
    Item::new(100, 3, 0),
];

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let boss = parse_input(&input);

    let (d1, _) = measure(DAY, "part 1", || solve_part_one(boss));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(boss));

    d1 + d2
}

#[derive(Debug, Copy, Clone)]
struct Character {
    hp: u32,
    damage: u32,
    armor: u32,
}

impl Character {
    fn new(hp: u32, damage: u32, armor: u32) -> Self {
        Character { hp, damage, armor }
    }

    fn turns_to_kill(self, other: Character) -> u32 {
        let attack_rate = self.damage.saturating_sub(other.armor).max(1);
        let mut turns_to_die = other.hp / attack_rate;
        if other.hp % attack_rate != 0 {
            turns_to_die += 1;
        }
        turns_to_die
    }
}

#[derive(Debug, Copy, Clone)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    const fn new(cost: u32, damage: u32, armor: u32) -> Self {
        Item {
            cost,
            damage,
            armor,
        }
    }
}

fn parse_input(input: &str) -> Character {
    let (mut hp, mut damage, mut armor) = (0, 0, 0);
    let mut checksum = 0;

    for line in input.lines() {
        if line.starts_with("Hit Points: ") {
            checksum |= 1 << 0;
            hp = line[12..].parse().unwrap();
        } else if line.starts_with("Damage: ") {
            checksum |= 1 << 1;
            damage = line[8..].parse().unwrap();
        } else if line.starts_with("Armor: ") {
            checksum |= 1 << 2;
            armor = line[7..].parse().unwrap();
        } else {
            panic!("Unexpected line: {}", line);
        }
    }

    assert_eq!(7, checksum);
    Character::new(hp, damage, armor)
}

fn solve_part_one(boss: Character) -> u32 {
    solve(
        boss,
        |old, new| old.cmp(&new),
        |boss_ttl, hero_ttl| boss_ttl.cmp(&hero_ttl),
    )
}

fn solve_part_two(boss: Character) -> u32 {
    solve(
        boss,
        |old, new| old.cmp(&new).reverse(),
        |boss_ttl, hero_ttl| boss_ttl.cmp(&hero_ttl).reverse(),
    )
}

fn solve<C, W>(boss: Character, cost_fn: C, win_fn: W) -> u32
where
    C: Copy + Fn(u32, u32) -> Ordering,
    W: Copy + Fn(u32, u32) -> Ordering,
{
    let mut cost = max_by(0, u32::max_value(), cost_fn);

    // everything is sorted by COST (increasing)
    for (ring_idx, r1) in RINGS.iter().enumerate() {
        let skip = if ring_idx == 0 {
            ring_idx
        } else {
            ring_idx + 1
        };

        for r2 in RINGS.iter().skip(skip) {
            for armor in ARMOR {
                for weapon in WEAPONS {
                    let hero = Character::new(
                        INITIAL_HP,
                        weapon.damage + armor.damage + r1.damage + r2.damage,
                        weapon.armor + armor.armor + r1.armor + r2.armor,
                    );

                    let boss_dies_in = hero.turns_to_kill(boss);
                    let hero_dies_in = boss.turns_to_kill(hero);

                    if win_fn(boss_dies_in, hero_dies_in) == Ordering::Less {
                        cost = min_by(cost, weapon.cost + armor.cost + r1.cost + r2.cost, cost_fn);
                    }
                }
            }
        }
    }

    cost
}

#[cfg(test)]
mod tests {
    use aoc_2015_common::input::default_test_input;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = load_input(default_test_input(DAY));
        let boss = parse_input(&input);
        let answer = solve_part_one(boss);
        assert_eq!(111, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let boss = parse_input(&input);
        let answer = solve_part_two(boss);
        assert_eq!(188, answer);
    }
}
