use std::{collections::BTreeMap, str::FromStr};

use advent_code_lib::{all_lines, simpler_main};
use anyhow::bail;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        Ok(())
    })
}

pub fn part1(filename: &str) -> anyhow::Result<i64> {
    let troop = MonkeyTroop::from_file(filename)?;
    Ok(troop.evaluate_root())
}

pub struct MonkeyTroop {
    name2index: BTreeMap<String,usize>,
    monkeys: Vec<Monkey>,
}

impl MonkeyTroop {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut start_map = BTreeMap::new();
        for line in all_lines(filename)? {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap();
            let name = &name[..4];
            start_map.insert(name.to_string(), parts.map(|p| p.to_string()).collect::<Vec<String>>());
        }
        let names_only: Vec<String> = start_map.keys().cloned().collect();
        let name2index: BTreeMap<String, usize> = names_only.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();

        let mut monkeys = vec![];
        for formula in start_map.values() {
            let monkey = match formula.len() {
                1 => Monkey::Value(formula[0].parse::<i64>()?),
                3 => Monkey::Oper(*name2index.get(&formula[0]).unwrap(), formula[1].parse::<Sym>()?, *name2index.get(&formula[2]).unwrap()),
                err => panic!("formula has {err} terms; not allowed")
            };
            monkeys.push(monkey);
        }
        Ok(Self {name2index, monkeys})
    }

    pub fn evaluate_root(&self) -> i64 {
        self.eval_monkey_at(*self.name2index.get("root").unwrap())
    }

    pub fn eval_monkey_at(&self, monkey: usize) -> i64 {
        self.monkeys[monkey].eval(self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Monkey {
    Value(i64),
    Oper(usize, Sym, usize)
}

impl Monkey {
    pub fn eval(&self, troop: &MonkeyTroop) -> i64 {
        match self {
            Self::Value(v) => *v,
            Self::Oper(m1, s, m2) => s.eval(troop.eval_monkey_at(*m1), troop.eval_monkey_at(*m2)),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Sym {
    Plus, Minus, Times, Divide
}

impl Sym {
    pub fn eval(&self, a: i64, b: i64) -> i64 {
        match self {
            Sym::Plus => a + b,
            Sym::Minus => a - b,
            Sym::Times => a * b,
            Sym::Divide => a / b,
        }
    }
}

impl FromStr for Sym {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Times,
            "/" => Self::Divide,
            _ => bail!("{s}: Not an operator"),
        })
    }
}