use std::path::Path;
use std::time::Duration;

use aoc_2015_common::input::load_input;
use aoc_2015_common::timing::measure;

const DAY: usize = 22;
const SPELLS: &[Spell] = &[
    Spell::new(53, 4, 0, 0, 0, 0, 0),
    Spell::new(73, 2, 0, 2, 0, 0, 0),
    Spell::new(113, 0, 7, 0, 0, 0, 6),
    Spell::new(173, 0, 0, 0, 3, 0, 6),
    Spell::new(229, 0, 0, 0, 0, 101, 5),
];

pub fn demo<P: AsRef<Path>>(path: P) -> Duration {
    let input = load_input(path);
    let boss = parse_input(&input);

    let (d1, _) = measure(DAY, "part 1", || solve_part_one(boss));
    let (d2, _) = measure(DAY, "part 2", || solve_part_two(boss));

    d1 + d2
}

#[derive(Debug, Copy, Clone)]
struct Boss {
    hp: u16,
    damage: u16,
}

impl Boss {
    const fn new(hp: u16, damage: u16) -> Self {
        Boss { hp, damage }
    }
}

#[derive(Debug, Copy, Clone)]
struct Mage {
    hp: u16,
    mana: u16,
}

impl Mage {
    const fn new(hp: u16, mana: u16) -> Self {
        Mage { hp, mana }
    }
}

#[derive(Debug, Copy, Clone)]
struct Spell {
    cost: u16,
    damage: u16,
    armor: u16,
    heal: u16,
    poison: u16,
    recharge: u16,
    duration: u8,
}

impl Spell {
    const fn new(
        cost: u16,
        damage: u16,
        armor: u16,
        heal: u16,
        poison: u16,
        recharge: u16,
        duration: u8,
    ) -> Self {
        Spell {
            cost,
            damage,
            armor,
            heal,
            poison,
            recharge,
            duration,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    effects: [u8; 5],
    mage: Mage,
    boss: Boss,
}

impl State {
    const fn new(boss: Boss) -> Self {
        State {
            effects: [0; 5],
            mage: Mage::new(50, 500),
            boss,
        }
    }
}

fn parse_input(input: &str) -> Boss {
    let (mut hp, mut damage) = (0, 0);
    let mut checksum = 0;

    for line in input.lines() {
        if line.starts_with("Hit Points: ") {
            checksum |= 1 << 0;
            hp = line[12..].parse().unwrap();
        } else if line.starts_with("Damage: ") {
            checksum |= 1 << 1;
            damage = line[8..].parse().unwrap();
        } else {
            panic!("Unexpected line: {}", line);
        }
    }

    assert_eq!(3, checksum);
    Boss::new(hp, damage)
}

fn solve_part_one(boss: Boss) -> Option<u16> {
    solve(boss, 0)
}

fn solve_part_two(boss: Boss) -> Option<u16> {
    solve(boss, 1)
}

fn solve(boss: Boss, handicap: u16) -> Option<u16> {
    let mut answer = None;

    for i in 0..SPELLS.len() {
        if let Some(cost) = try_spell(i, State::new(boss), handicap) {
            answer = answer.map_or(Some(cost), |answer: u16| Some(answer.min(cost)));
        }
    }

    answer
}

fn try_spell(idx: usize, mut state: State, handicap: u16) -> Option<u16> {
    // Handle "part 2"
    state.mage.hp = state.mage.hp.saturating_sub(handicap);
    if state.mage.hp == 0 {
        return None;
    }

    //apply effects at the beginning of the mage's turn
    for idx in 0..state.effects.len() {
        let spell = SPELLS[idx];

        if state.effects[idx] > 0 {
            state.effects[idx] -= 1;

            state.mage.mana = state.mage.mana.saturating_add(spell.recharge);
            state.boss.hp = state.boss.hp.saturating_sub(spell.poison);
        }
    }

    let spell = SPELLS[idx];

    // Not enough mana to cast a spell
    if spell.cost > state.mage.mana {
        return None;
    }

    // Cannot cast a spell for a second time while its effect is still active
    if state.effects[idx] > 1 {
        return None;
    }

    // mage's turn
    state.mage.mana = state.mage.mana.saturating_sub(spell.cost);
    state.mage.hp = state.mage.hp.saturating_add(spell.heal);
    state.boss.hp = state.boss.hp.saturating_sub(spell.damage);
    state.effects[idx] = spell.duration;

    //apply effects at the beginning of the boss' turn
    let mut armor_bonus = 0u16;
    for idx in 0..state.effects.len() {
        let spell = SPELLS[idx];

        if state.effects[idx] > 0 {
            state.effects[idx] -= 1;

            state.mage.mana = state.mage.mana.saturating_add(spell.recharge);
            state.boss.hp = state.boss.hp.saturating_sub(spell.poison);
            armor_bonus = armor_bonus.saturating_add(spell.armor);
        }
    }

    // The mage won
    if state.boss.hp == 0 {
        return Some(spell.cost);
    }

    // boss' turn
    let boss_damage = 1.max(state.boss.damage.saturating_sub(armor_bonus));
    state.mage.hp = state.mage.hp.saturating_sub(boss_damage);

    // The boss won
    if state.mage.hp == 0 {
        return None;
    }

    let mut total_cost = None;
    for idx in 0..SPELLS.len() {
        if let Some(cost) = try_spell(idx, state, handicap) {
            match total_cost {
                None => total_cost = Some(cost),
                Some(tc) => total_cost = Some(tc.min(cost)),
            }
        }
    }

    total_cost.map(|c| c + spell.cost)
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
        assert_eq!(Some(1824), answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_input(default_test_input(DAY));
        let boss = parse_input(&input);

        let answer = solve_part_two(boss);
        assert_eq!(Some(1937), answer);
    }
}
