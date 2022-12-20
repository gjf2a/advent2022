use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    ops::Index,
};

use advent_code_lib::{all_lines, simpler_main};
use bare_metal_modulo::*;

const DECRYPTION_KEY: i64 = 811589153;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let nums = TrackedNums::from_file(filename)?;
        println!("Part 1: {}", part1(&nums));
        println!("Part 2: {}", part2(&nums));
        Ok(())
    })
}

pub fn part1(nums: &TrackedNums) -> i64 {
    let mut nums = nums.clone();
    nums.mix();
    nums.coordinate_sum()
}

pub fn part2(nums: &TrackedNums) -> i64 {
    let mut nums = nums.clone();
    nums.scalar_multiply(DECRYPTION_KEY);
    for _ in 0..10 {
        nums.mix();
    }
    nums.coordinate_sum()
}

#[derive(Debug, Clone)]
pub struct TrackedNums {
    nums: VecDeque<(i64, usize)>,
    index2num: VecDeque<usize>,
}

impl TrackedNums {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let nums: VecDeque<(i64, usize)> = load_nums(filename)?
            .iter()
            .copied()
            .enumerate()
            .map(|(n, i)| (i, n))
            .collect();
        let index2num: VecDeque<usize> = (0..nums.len()).collect();
        Ok(Self { nums, index2num })
    }

    pub fn scalar_multiply(&mut self, scalar: i64) {
        for (num, _) in self.nums.iter_mut() {
            *num *= scalar;
        }
    }

    pub fn coordinate_sum(&self) -> i64 {
        let zero_index = self.find(0).unwrap();
        (1000..=3000)
            .step_by(1000)
            .map(|n| self[zero_index + n])
            .inspect(|n| println!("{n}"))
            .sum()
    }

    pub fn assert_unique(&self) {
        assert_eq!(
            self.nums.iter().copied().collect::<BTreeSet<_>>().len(),
            self.nums.len()
        );
    }

    pub fn find(&self, value: i64) -> Option<usize> {
        self.nums
            .iter()
            .enumerate()
            .find(|(_, n)| n.0 == value)
            .map(|(i, _)| i)
    }

    pub fn nums(&self) -> Vec<i64> {
        self.nums.iter().map(|(n, _)| n).copied().collect()
    }

    pub fn mix(&mut self) {
        for i in 0..self.len() {
            self.rotate(i);
        }
    }

    pub fn rotate(&mut self, start_index: usize) {
        let i = self.index2num[start_index];
        let mut steps_left = ModNum::new(self[i].abs(), self.len() as i64 - 1).a();
        let update = if self[i] < 0 { -1 } else { 1 };
        let mut current = i;
        while steps_left > 0 {
            let start = current;
            let end = (ModNum::new(start as isize, self.len() as isize) + update).a() as usize;
            self.swap(start, end, &mut current);
            steps_left -= 1;
        }
    }

    pub fn swap(&mut self, start: usize, end: usize, current: &mut usize) {
        self.nums.swap(start, end);
        self.align(start);
        self.align(end);
        *current = end;
    }

    fn align(&mut self, i: usize) {
        self.index2num[self.nums[i].1] = i;
    }

    pub fn len(&self) -> usize {
        self.nums.len()
    }
}

impl Index<usize> for TrackedNums {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        let index = ModNum::new(index, self.len());
        &self.nums[index.a()].0
    }
}

pub fn load_nums(filename: &str) -> anyhow::Result<VecDeque<i64>> {
    Ok(all_lines(filename)?
        .map(|line| line.parse().unwrap())
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::TrackedNums;

    #[test]
    fn test_mix() {
        let mut nums = TrackedNums::from_file("ex/day20.txt").unwrap();
        nums.mix();
        assert_eq!(vec![-2, 1, 2, -3, 4, 0, 3], nums.nums());
        nums.mix();
        println!("{:?}", nums.nums());
    }
}
