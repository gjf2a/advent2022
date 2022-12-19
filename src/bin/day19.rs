use std::{collections::BTreeMap, iter::zip};

use advent_code_lib::{all_lines, all_nums_from, simpler_main};
use enum_iterator::{all, Sequence};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let costs = Costs::from_file(filename)?;
        println!("Part 1: {}", part1(&costs));
        Ok(())
    })
}

pub fn part1(costs: &Costs) -> usize {
    println!("{costs:?}");
    0
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Sequence)]
pub enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug)]
pub struct Costs {
    table: Vec<BTreeMap<Mineral, BTreeMap<Mineral, i64>>>,
}

impl Costs {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut table = Vec::new();
        for line in all_lines(filename)? {
            let mut nums = all_nums_from(line);
            nums.pop_front().unwrap();
            let mut this_table = BTreeMap::new();
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
                this_table.insert(mineral, cost.iter().copied().collect());
            }
            table.push(this_table);
        }
        Ok(Self { table })
    }
}
