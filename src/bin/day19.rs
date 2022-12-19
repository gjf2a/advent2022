use std::{
    cmp::{max, min},
    collections::HashSet,
    iter::zip,
};

use advent_code_lib::{all_lines, all_nums_from, simpler_main};
use enum_iterator::{all, reverse_all, Sequence};
use enum_map::{Enum, EnumMap};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let costs = Costs::from_file(filename)?;
        println!("Part 1: {}", part1(&costs));
        println!("Part 2: {}", part2(&costs));
        Ok(())
    })
}

pub fn part1(costs: &Costs) -> usize {
    costs.part_1_score(24)
}

pub fn part2(costs: &Costs) -> usize {
    costs.part_2_score(32)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Enum, Sequence)]
pub enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Default)]
pub struct BlueprintStateTable {
    states: Vec<HashSet<State>>,
}

impl BlueprintStateTable {
    pub fn after(blueprint: usize, costs: &Costs, minutes: usize) -> Self {
        let mut states: Vec<HashSet<State>> = vec![[State::default()].iter().cloned().collect()];
        for minute in 1..=minutes {
            let mut new_states = HashSet::new();
            let mut most_geodes_produced = 0;
            for state in states[minute - 1].iter() {
                for successor in state.successors(blueprint, costs) {
                    let revised_geo =
                        successor.geode_production_upper_bound(minutes - minute, blueprint, costs);
                    if revised_geo > most_geodes_produced {
                        most_geodes_produced = max(successor.geodes_mined(), most_geodes_produced);
                        new_states.insert(successor);
                    }
                }
            }
            states.push(new_states);
        }
        Self { states }
    }

    pub fn geodes(&self) -> usize {
        self.states
            .last()
            .unwrap()
            .iter()
            .map(|s| s.mined_minerals[Mineral::Geode])
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    robot_count: EnumMap<Mineral, usize>,
    mined_minerals: EnumMap<Mineral, usize>,
}

impl Default for State {
    fn default() -> Self {
        let mut robot_count = EnumMap::default();
        robot_count[Mineral::Ore] = 1;
        State {
            robot_count,
            mined_minerals: EnumMap::default(),
        }
    }
}

impl State {
    pub fn successors(&self, blueprint: usize, costs: &Costs) -> Vec<Self> {
        let mut result = vec![];
        for robot in reverse_all::<Mineral>() {
            if let Some(after_use) = costs.construct(blueprint, robot, &self.mined_minerals) {
                let mut successor = Self {
                    robot_count: self.robot_count.clone(),
                    mined_minerals: after_use,
                };
                successor.mine();
                successor.robot_count[robot] += 1;
                match robot {
                    Mineral::Geode | Mineral::Obsidian => return vec![successor],
                    _ => {
                        result.push(successor);
                    }
                }
            }
        }
        let mut no_build = self.clone();
        no_build.mine();
        result.push(no_build);
        result
    }

    pub fn mine(&mut self) {
        for robot in all::<Mineral>() {
            self.mined_minerals[robot] += self.robot_count[robot];
        }
    }

    pub fn geodes_mined(&self) -> usize {
        self.mined_minerals[Mineral::Geode]
    }

    pub fn production_upper_bound_for(
        &self,
        max_extra_robots: usize,
        mineral: Mineral,
        minutes_left: usize,
    ) -> usize {
        let mut current_mineral = self.mined_minerals[mineral];
        let mut current_robots = self.robot_count[mineral];
        for _ in 0..minutes_left {
            current_mineral += current_robots;
            if current_robots < self.robot_count[mineral] + max_extra_robots {
                current_robots += 1;
            }
        }
        current_mineral
    }

    pub fn original_geode(&self, minutes_left: usize) -> usize {
        self.production_upper_bound_for(minutes_left, Mineral::Geode, minutes_left)
    }

    pub fn geode_production_upper_bound(
        &self,
        minutes_left: usize,
        blueprint: usize,
        costs: &Costs,
    ) -> usize {
        let ore_upper_bound = self.production_upper_bound_for(0, Mineral::Ore, minutes_left);
        let max_clay_robots = ore_upper_bound / costs.table[blueprint][Mineral::Clay][Mineral::Ore];
        let clay_upper_bound =
            self.production_upper_bound_for(max_clay_robots, Mineral::Clay, minutes_left);
        let max_obsidian_robots =
            clay_upper_bound / costs.table[blueprint][Mineral::Obsidian][Mineral::Clay];
        let obsidian_upper_bound =
            self.production_upper_bound_for(max_obsidian_robots, Mineral::Obsidian, minutes_left);
        let max_geode_robots =
            obsidian_upper_bound / costs.table[blueprint][Mineral::Geode][Mineral::Obsidian];
        self.production_upper_bound_for(max_geode_robots, Mineral::Geode, minutes_left)
    }
}

#[derive(Clone, Debug)]
pub struct Costs {
    table: Vec<EnumMap<Mineral, EnumMap<Mineral, usize>>>,
}

impl Costs {
    pub fn part_1_score(&self, minutes: usize) -> usize {
        let mut total = 0;
        for blueprint in 0..self.table.len() {
            let table = BlueprintStateTable::after(blueprint, self, minutes);
            let id = blueprint + 1;
            let geodes = table.geodes();
            let score = id * geodes;
            println!("Blueprint {id} geodes: {geodes} ({score})");
            total += score;
        }
        total
    }

    pub fn part_2_score(&self, minutes: usize) -> usize {
        let mut total = 1;
        for blueprint in 0..min(3, self.table.len()) {
            let table = BlueprintStateTable::after(blueprint, self, minutes);
            let id = blueprint + 1;
            let geodes = table.geodes();
            println!("Blueprint {id} geodes: {geodes}");
            total *= geodes;
        }
        total
    }

    pub fn construct(
        &self,
        blueprint: usize,
        robot: Mineral,
        mined_minerals: &EnumMap<Mineral, usize>,
    ) -> Option<EnumMap<Mineral, usize>> {
        let mut result = mined_minerals.clone();
        for (mineral, cost) in self.table[blueprint][robot].iter() {
            if *cost > result[mineral] {
                return None;
            } else {
                result[mineral] -= cost;
            }
        }
        Some(result)
    }

    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut table = Vec::new();
        for line in all_lines(filename)? {
            let mut nums = all_nums_from(line);
            nums.pop_front().unwrap();
            let mut this_table = EnumMap::default();
            let costs = [
                vec![(Mineral::Ore, nums.pop_front().unwrap())],
                vec![(Mineral::Ore, nums.pop_front().unwrap())],
                vec![
                    (Mineral::Ore, nums.pop_front().unwrap()),
                    (Mineral::Clay, nums.pop_front().unwrap()),
                ],
                vec![
                    (Mineral::Ore, nums.pop_front().unwrap()),
                    (Mineral::Obsidian, nums.pop_front().unwrap()),
                ],
            ];
            for (mineral, cost) in zip(all::<Mineral>(), costs.iter()) {
                this_table[mineral] = cost.iter().copied().collect();
            }
            table.push(this_table);
        }
        Ok(Self { table })
    }
}
