use std::{ops::Index, collections::{VecDeque, BTreeSet, BTreeMap}};

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
pub struct FromZero {
    order_of_arrival: BTreeMap<usize,usize>,
    order_from_zero: VecDeque<(i64, usize)>,
}

impl Index<usize> for FromZero {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        let index = ModNum::new(index, self.len());
        &self.order_from_zero[index.a()].0
    }
}

impl FromZero {
    pub fn coordinate_sum(&self) -> i64 {
        (1000..=3000)
            .step_by(1000)
            .map(|n| self[n])
            .inspect(|n| println!("{n}"))
            .sum()
    }

    pub fn mix(&mut self) {
        for arrival in 0..self.len() {
            self.rotate(arrival);
        }
    }

    pub fn scalar_multiply(&mut self, scalar: i64) {
        for (v, _) in self.order_from_zero.iter_mut() {
            *v *= scalar;
        }
    }

    fn rotate(&mut self, arrival: usize) {
        let from_zero = self.order_of_arrival.get(&arrival).unwrap();
        let (value, _) = self.order_from_zero[*from_zero];
        let direction = if value < 0 {-1} else {1};
        let mut i = ModNum::new(*from_zero as isize, self.len() as isize);
        for _ in 0..value.abs() {
            let origin = i.a() as usize;
            i += direction;
            if i < 0 {
                if direction < 0 {
                    let popped = self.order_from_zero.pop_front().unwrap();
                    self.order_from_zero.push_back(popped);
                    for (_,i) in self.order_of_arrival.iter_mut() {
                        *i -= 1;
                    }
                } else {
                    let popped = self.order_from_zero.pop_back().unwrap();
                    self.order_from_zero.push_front(popped);
                    for (_,i) in self.order_of_arrival.iter_mut() {
                        *i += 1;
                    }
                }
            } else {
                let destination = i.a() as usize;
                *self.order_of_arrival.get_mut(&self.order_from_zero[origin].1).unwrap() = destination;
                *self.order_of_arrival.get_mut(&self.order_from_zero[destination].1).unwrap() = origin;
                self.order_from_zero.swap(origin, destination);
            }
        }
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.order_from_zero.len(), self.order_of_arrival.len());
        self.order_from_zero.len()
    }

    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let nums: Vec<i64> = all_lines(filename)?.map(|n| n.parse().unwrap()).collect();
        let mut zero = ModNum::new(0, nums.len());
        while nums[zero.a()] != 0 {
            zero += 1;
        }
        let mut order_from_zero = VecDeque::new();
        let mut order_of_arrival = BTreeMap::new();
        while order_from_zero.len() < nums.len() {
            order_of_arrival.insert(zero.a(), order_from_zero.len());
            order_from_zero.push_back((nums[zero.a()], zero.a()));
            zero += 1;
        }
        Ok(Self {order_from_zero, order_of_arrival})
    }
}

#[derive(Debug, Clone)]
pub struct TrackedNums {
    nums: VecDeque<(i64,usize)>,
    index2num: VecDeque<usize>,
}

impl TrackedNums {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let nums: VecDeque<(i64, usize)> = load_nums(filename)?.iter().copied().enumerate().map(|(n,i)| (i, n)).collect();
        let index2num: VecDeque<usize> = (0..nums.len()).collect();
        Ok(Self {nums, index2num })
    }

    pub fn scalar_multiply(&mut self, scalar: i64) {
        for (num,_) in self.nums.iter_mut() {
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
        assert_eq!(self.nums.iter().copied().collect::<BTreeSet<_>>().len(), self.nums.len());
    }

    pub fn find(&self, value: i64) -> Option<usize> {
        self.nums.iter().enumerate().find(|(_, n)| n.0 == value).map(|(i,_)| i)
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
        // A worthy but incorrect attempt
        let mut steps_left = ModNum::new(self[i].abs(), self.len() as i64 - 1).a();
        //let mut steps_left = self[i].abs();
        let update = if self[i] < 0 {-1} else {1};
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
    Ok(all_lines(filename)?.map(|line| line.parse().unwrap()).collect())
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

/*
Initial arrangement:
1, 2, -3, 3, -2, 0, 4
0, 1,  2, 3,  4, 5, 6

1 moves between 2 and -3:
2, 1, -3, 3, -2, 0, 4
1, 0,  2, 3,  4, 5, 6

2 moves between -3 and 3:
1, -3, 2, 3, -2, 0, 4
0, 2,  1, 3,  4, 5, 6

-3 moves between -2 and 0:
1, 2, 3, -2, -3, 0, 4
0, 1, 4,  2,  3, 5, 6

3 moves between 0 and 4:
1, 2, -2, -3, 0, 3, 4

-2 moves between 4 and 1:
1, 2, -3, 0, 3, 4, -2

0 does not move:
1, 2, -3, 0, 3, 4, -2

4 moves between -3 and 0:
1, 2, -3, 4, 0, 3, -2
 */