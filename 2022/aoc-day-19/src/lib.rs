use std::collections::VecDeque;
use std::hash::Hash;
use std::str::FromStr;

use aoc_shared::hashing::HashSet;
use once_cell::sync::Lazy;
use regex::Regex;

const RES_ORE: usize = 0;
const RES_CLY: usize = 1;
const RES_OBS: usize = 2;

const RBT_ORE: usize = 0;
const RBT_CLY: usize = 1;
const RBT_OBS: usize = 2;
const RBT_GEO: usize = 3;

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        "Blueprint (\\d+): \
       Each ore robot costs (\\d+) ore. \
       Each clay robot costs (\\d+) ore. \
       Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
       Each geode robot costs (\\d+) ore and (\\d+) obsidian.\
   ",
    )
    .unwrap()
});

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct OreRobot {
    ore: u16,
}

impl OreRobot {
    fn can_build(self, resources: [u16; 3]) -> bool {
        self.ore <= resources[RES_ORE]
    }

    fn build(self, mut resources: [u16; 3], mut robots: [u16; 4]) -> ([u16; 3], [u16; 4]) {
        resources[RES_ORE] -= self.ore;
        robots[RBT_ORE] += 1;

        (resources, robots)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ClayRobot {
    ore: u16,
}

impl ClayRobot {
    fn can_build(self, resources: [u16; 3]) -> bool {
        self.ore <= resources[RES_ORE]
    }

    fn build(self, mut resources: [u16; 3], mut robots: [u16; 4]) -> ([u16; 3], [u16; 4]) {
        resources[RES_ORE] -= self.ore;
        robots[RBT_CLY] += 1;

        (resources, robots)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ObsidianRobot {
    ore: u16,
    clay: u16,
}

impl ObsidianRobot {
    fn can_build(self, resources: [u16; 3]) -> bool {
        self.ore <= resources[RES_ORE] && self.clay <= resources[RES_CLY]
    }

    fn build(self, mut resources: [u16; 3], mut robots: [u16; 4]) -> ([u16; 3], [u16; 4]) {
        resources[RES_ORE] -= self.ore;
        resources[RES_CLY] -= self.clay;
        robots[RBT_OBS] += 1;

        (resources, robots)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct GeodeRobot {
    ore: u16,
    obsidian: u16,
}

impl GeodeRobot {
    fn can_build(self, resources: [u16; 3]) -> bool {
        self.ore <= resources[RES_ORE] && self.obsidian <= resources[RES_OBS]
    }

    fn build(self, mut resources: [u16; 3], mut robots: [u16; 4]) -> ([u16; 3], [u16; 4]) {
        resources[RES_ORE] -= self.ore;
        resources[RES_OBS] -= self.obsidian;
        robots[RBT_GEO] += 1;

        (resources, robots)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct BluePrint {
    id: u16,
    max_res: [u16; 3],
    ore_robot: OreRobot,
    clay_robot: ClayRobot,
    obsidian_robot: ObsidianRobot,
    geode_robot: GeodeRobot,
}

impl FromStr for BluePrint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let capture = REGEX
            .captures(s)
            .ok_or_else(|| format!("failed to parse: {}", s))?;

        let id = capture[1]
            .parse()
            .map_err(|_| format!("failed to parse capture: {}", &capture[1]))?;

        let ore_robot = OreRobot {
            ore: capture[2]
                .parse()
                .map_err(|_| format!("failed to parse capture: {}", &capture[2]))?,
        };

        let clay_robot = ClayRobot {
            ore: capture[3]
                .parse()
                .map_err(|_| format!("failed to parse capture: {}", &capture[3]))?,
        };

        let obsidian_robot = ObsidianRobot {
            ore: capture[4]
                .parse()
                .map_err(|_| format!("failed to parse capture: {}", &capture[4]))?,

            clay: capture[5]
                .parse()
                .map_err(|_| format!("failed to parse capture: {}", &capture[5]))?,
        };

        let geode_robot = GeodeRobot {
            ore: capture[6]
                .parse()
                .map_err(|_| format!("failed to parse capture: {}", &capture[6]))?,
            obsidian: capture[7]
                .parse()
                .map_err(|_| format!("failed to parse capture: {}", &capture[7]))?,
        };

        let mut max_res = [0; 3];
        max_res[RES_ORE] = ore_robot
            .ore
            .max(clay_robot.ore)
            .max(obsidian_robot.ore)
            .max(geode_robot.ore);
        max_res[RES_CLY] = obsidian_robot.clay;
        max_res[RES_OBS] = geode_robot.obsidian;

        Ok(BluePrint {
            id,
            max_res,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        })
    }
}

pub fn part_one(blue_prints: &[BluePrint]) -> u16 {
    bfs(blue_prints, 24, blue_prints.len(), 0, |acc, id, geodes| {
        acc + id * geodes
    })
}

pub fn part_two(blue_prints: &[BluePrint]) -> u16 {
    bfs(blue_prints, 32, 3, 1, |acc, _, geodes| acc * geodes)
}

fn bfs<F: Fn(u16, u16, u16) -> u16>(
    blue_prints: &[BluePrint],
    minutes: u32,
    take: usize,
    initial: u16,
    fold_fn: F,
) -> u16 {
    let mut visited = HashSet::default();
    let mut answer = initial;

    let mut queue = VecDeque::new();
    for blue_print in blue_prints.iter().copied().take(take) {
        visited.clear();
        queue.clear();

        queue.push_back(([1, 0, 0, 0], [0, 0, 0], 0));
        visited.insert(([1, 0, 0, 0], [0, 0, 0]));

        let mut best = 0;
        for _ in 0..minutes {
            for _ in 0..queue.len() {
                let (robots, resources, geodes) = queue.pop_front().unwrap();
                let geodes = geodes + robots[RBT_GEO];
                best = best.max(geodes);

                // HEURISTIC: prune branches that have lower number of geodes
                // What's the correct value?
                if best - geodes > 0 {
                    continue;
                }

                // We always build a geode robot if we can and avoid branching
                if blue_print.geode_robot.can_build(resources) {
                    let (mut res, rob) = blue_print.geode_robot.build(resources, robots);
                    res = update_resources(res, robots);

                    if visited.insert((rob, res)) {
                        queue.push_back((rob, res, geodes));
                    }
                    continue;
                }

                if blue_print.max_res[RES_OBS] > robots[RBT_OBS] {
                    if blue_print.obsidian_robot.can_build(resources) {
                        let (mut res, rob) = blue_print.obsidian_robot.build(resources, robots);
                        res = update_resources(res, robots);

                        if visited.insert((rob, res)) {
                            queue.push_back((rob, res, geodes));
                        }
                    }
                }

                if blue_print.max_res[RES_CLY] > robots[RBT_CLY] {
                    if blue_print.clay_robot.can_build(resources) {
                        let (mut res, rob) = blue_print.clay_robot.build(resources, robots);
                        res = update_resources(res, robots);

                        if visited.insert((rob, res)) {
                            queue.push_back((rob, res, geodes));
                        }
                    }
                }

                if blue_print.max_res[RES_ORE] > robots[RBT_ORE] {
                    if blue_print.ore_robot.can_build(resources) {
                        let (mut res, rob) = blue_print.ore_robot.build(resources, robots);
                        res = update_resources(res, robots);

                        if visited.insert((rob, res)) {
                            queue.push_back((rob, res, geodes));
                        }
                    }
                }

                let resources = update_resources(resources, robots);
                if visited.insert((robots, resources)) {
                    queue.push_back((robots, resources, geodes));
                }
            }
        }

        answer = fold_fn(answer, blue_print.id, best);
    }

    answer
}

fn update_resources(mut resources: [u16; 3], robots: [u16; 4]) -> [u16; 3] {
    resources[RES_ORE] += robots[RBT_ORE];
    resources[RES_CLY] += robots[RBT_CLY];
    resources[RES_OBS] += robots[RBT_OBS];
    resources
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_line_delimited_input_from_file;

    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_one(&input);

        assert_eq!(817, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_line_delimited_input_from_file("inputs/input.txt");
        let answer = part_two(&input);

        assert_eq!(4216, answer);
    }
}
