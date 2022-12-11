use std::fmt::Debug;
use std::{collections::VecDeque, str::FromStr};

use advent_code_lib::{all_lines, simpler_main};
use anyhow::bail;
use bare_metal_modulo::*;

// Expected result on example: 2713310158
// Actual:                     2714454444

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let troop1 = MonkeyTroop::from_file(filename, 3)?;
        println!("Part 1: {}", evaluate(troop1, 20));
        let troop2 = MonkeyTroop::from_file(filename, 1)?;
        println!("Part 2: {}", evaluate(troop2, 10000));
        Ok(())
    })
}

pub fn evaluate(mut monkeys: MonkeyTroop, total_rounds: usize) -> u128 {
    for _ in 0..total_rounds {
        monkeys.round();
    }
    monkeys.monkey_business()
}

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Plus,
    Times,
}

impl OpCode {
    fn eval(&self, left: ModNum<i64>, right: ModNum<i64>) -> ModNum<i64> {
        match self {
            OpCode::Plus => left + right,
            OpCode::Times => left * right,
        }
    }
}

impl FromStr for OpCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            "*" => Ok(Self::Times),
            _ => bail!("{s}: Not supported"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    left: Option<i64>,
    right: Option<i64>,
    op: OpCode,
}

impl Operation {
    pub fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        assert_eq!(parts.next().unwrap(), "Operation:");
        assert_eq!(parts.next().unwrap(), "new");
        assert_eq!(parts.next().unwrap(), "=");
        let left = parts.next().unwrap().parse::<i64>().ok();
        let op = parts.next().unwrap().parse::<OpCode>().unwrap();
        let right = parts.next().unwrap().parse::<i64>().ok();
        Operation { left, right, op }
    }

    pub fn eval_on(&self, old: ModNum<i64>) -> ModNum<i64> {
        self.op.eval(Self::convert(self.left, old), Self::convert(self.right, old))
    }

    pub fn convert(value: Option<i64>, old: ModNum<i64>) -> ModNum<i64> {
        value.map_or(old, |v| ModNum::new(v, old.m()))
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<ModNum<i64>>,
    op: Operation,
    div_test_value: i64,
    true_monkey: usize,
    false_monkey: usize,
    total_inspections: u128,
}

fn keep_only<F: Fn(char) -> bool>(check: F, s: String) -> String {
    s.chars().map(|c| if check(c) { c } else { ' ' }).collect()
}

fn keep_digits(s: String) -> String {
    keep_only(|c| c.is_digit(10), s)
}

fn one_num_from<N: FromStr>(s: String) -> N {
    keep_digits(s)
        .split_whitespace()
        .next()
        .unwrap()
        .parse::<N>()
        .ok()
        .unwrap()
}

fn all_nums_from<N: FromStr>(s: String) -> VecDeque<N> {
    keep_digits(s)
        .split_whitespace()
        .map(|s| s.parse::<N>().ok().unwrap())
        .collect()
}

impl Monkey {
    pub fn from_lines<I: Iterator<Item = String>>(lines: &mut I) -> Option<Self> {
        let line1 = lines.next();
        if line1.is_some() {
            let items: VecDeque<i64> = all_nums_from(lines.next().unwrap());
            let product = items.iter().product();
            let items = items.iter().map(|n| ModNum::new(*n, product)).collect();
            let op = Operation::from(lines.next().unwrap().as_str());
            let div_test_value = one_num_from::<i64>(lines.next().unwrap());
            let true_monkey = one_num_from::<usize>(lines.next().unwrap());
            let false_monkey = one_num_from::<usize>(lines.next().unwrap());
            lines.next();
            let total_inspections = 0;
            Some(Self {
                items,
                op,
                div_test_value,
                true_monkey,
                false_monkey,
                total_inspections,
            })
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct MonkeyTroop {
    monkeys: Vec<Monkey>,
    worry_div: i64
}

impl MonkeyTroop {
    pub fn from_file(filename: &str, worry_div: i64) -> anyhow::Result<MonkeyTroop> {
        let mut monkeys = vec![];
        let mut lines = all_lines(filename)?;
        loop {
            if let Some(monkey) = Monkey::from_lines(&mut lines) {
                monkeys.push(monkey);
            } else {
                let gcf = monkeys.iter().map(|m| m.div_test_value).product();
                for monkey in monkeys.iter_mut() {
                    for item in monkey.items.iter_mut() {
                        *item = ModNum::new(item.a(), gcf);
                    }
                }
                return Ok(Self {monkeys, worry_div});
            }
        }
    }

    pub fn monkey_business(&self) -> u128 {
        let mut scores: Vec<u128> = self.monkeys.iter().map(|m| m.total_inspections).collect();
        scores.sort_by(|a, b| b.cmp(a));
        scores[0] * scores[1]
    }

    pub fn throw_first(&mut self, monkey: usize) {
        if let Some(mut worry) = self.monkeys[monkey].items.pop_front() {
            worry = self.monkeys[monkey].op.eval_on(worry);
            worry = (worry / self.worry_div).unwrap();
            let test = ModNum::new(worry.a(), self.monkeys[monkey].div_test_value) == 0;
            let target = if test {self.monkeys[monkey].true_monkey} else {self.monkeys[monkey].false_monkey};
            self.monkeys[target].items.push_back(worry);
            self.monkeys[monkey].total_inspections += 1;
        }
    }

    pub fn throw_all(&mut self, monkey: usize) {
        while !self.monkeys[monkey].items.is_empty() {
            self.throw_first(monkey);
        }
    }

    pub fn round(&mut self) {
        for monkey in 0..self.monkeys.len() {
            self.throw_all(monkey);
        }
    }
}