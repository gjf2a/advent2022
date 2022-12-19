use std::{iter::zip, collections::HashSet, cmp::{max, min}};

use advent_code_lib::{all_lines, all_nums_from, simpler_main};
use enum_iterator::{all, Sequence};
use enum_map::{EnumMap, Enum};

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

// This cries out for dynamic programming.
// 
// What is in a state?
// * elapsed time
// * number of each robot type
// * amount of each collected mineral
#[derive(Default)]
pub struct BlueprintStateTable {
    states: Vec<HashSet<State>>
}

impl BlueprintStateTable {
    pub fn after(blueprint: usize, costs: &Costs, minutes: usize) -> Self {
        let mut states: Vec<HashSet<State>> = vec![[State::default()].iter().cloned().collect()];
        for minute in 1..=minutes {
            let mut new_states = HashSet::new();
            let mut insertions = 0;
            let mut most_geodes_produced = 0;
            for state in states[minute - 1].iter() {
                for successor in state.successors(blueprint, costs) {
                    if successor.geode_production_upper_bound(minutes - minute) > most_geodes_produced {
                        most_geodes_produced = max(successor.geodes_mined(), most_geodes_produced);
                        new_states.insert(successor);
                        insertions += 1;
                    }
                }
            }
            println!("minute {minute}: new states: {} ({insertions})", new_states.len());
            states.push(new_states);
        }
        Self {states}
    }

    pub fn geodes(&self) -> usize {
        self.states.last().unwrap().iter().map(|s| s.mined_minerals[Mineral::Geode]).max().unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    elapsed_minutes: usize,
    robot_count: EnumMap<Mineral, usize>,
    mined_minerals: EnumMap<Mineral, usize>,
}

impl Default for State {
    fn default() -> Self {
        let mut robot_count = EnumMap::default();
        robot_count[Mineral::Ore] = 1;
        State {elapsed_minutes: 0, robot_count, mined_minerals: EnumMap::default()}
    }
}

impl State {
    pub fn successors(&self, blueprint: usize, costs: &Costs) -> Vec<Self> {
        let mut result = vec![self.clone()];
        result[0].mine();
        for robot in all::<Mineral>() {
            if let Some(after_use) = costs.construct(blueprint, robot, &self.mined_minerals) {
                let mut successor = Self {elapsed_minutes: self.elapsed_minutes, robot_count: self.robot_count.clone(), mined_minerals: after_use};
                successor.mine();
                successor.robot_count[robot] += 1;
                result.push(successor);
            }
        }
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

    pub fn geode_production_upper_bound(&self, minutes_left: usize) -> usize {
        let mut current_geodes = self.mined_minerals[Mineral::Geode];
        let mut current_geode_robots = self.robot_count[Mineral::Geode];
        for _ in 0..minutes_left {
            current_geodes += current_geode_robots;
            current_geode_robots += 1;
        }        
        current_geodes
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

    pub fn construct(&self, blueprint: usize, robot: Mineral, mined_minerals: &EnumMap<Mineral, usize>) -> Option<EnumMap<Mineral, usize>> {
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
