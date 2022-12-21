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

/*pub fn part2(filename: &str) -> anyhow::Result<i64> {
    let troop = MonkeyTroop::from_file(filename)?;
    Ok(troop.find_human_yell())
}*/

#[derive(Clone)]
pub struct MonkeyTroop {
    monkeys: BTreeMap<String,Monkey>,
    root_name: String,
}

impl MonkeyTroop {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut monkeys = BTreeMap::new();
        for line in all_lines(filename)? {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap();
            let name = &name[..4];
            let formula: Vec<String> = parts.map(|p| p.to_string()).collect();
            let monkey = match formula.len() {
                1 => Monkey::Value(formula[0].parse::<i64>()?),
                3 => Monkey::Oper(formula[0].clone(), formula[1].parse::<Sym>()?, formula[2].clone()),
                err => panic!("formula has {err} terms; not allowed")
            };
            monkeys.insert(name.to_owned(), monkey);
        }
        Ok(Self {root_name: "root".to_owned(), monkeys})
    }

    fn root_monkey(&self) -> Monkey {
        self.monkeys.get(self.root_name.as_str()).unwrap().clone()
    }

/* 
    pub fn find_human_yell(&self) -> i64 {
        let (t1, t2) = self.split_troop();
        let (human_troop, other_troop) = if t1.has_human() {(&t1, &t2)} else {(&t2, &t1)};
        let goal = other_troop.evaluate_root();
        human_troop.solve_human()
    }
    
    pub fn has_human(&self) -> bool {
        self.name2index.contains_key("humn")
    }

    fn solve_human(&self) -> i64 {

    }

    pub fn split_troop(&self) -> (Self, Self) {
        let mut left = self.clone();
        let mut right = self.clone();
        let root_monkey = self.root_monkey();
        left.root_name = 
        (left, right)
    }
    */

    pub fn evaluate_root(&self) -> i64 {
        self.root_monkey().eval(self)
    }

    pub fn eval_monkey_at(&self, monkey_name: &str) -> i64 {
        self.monkeys.get(monkey_name).unwrap().eval(self)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Monkey {
    Value(i64),
    Oper(String, Sym, String)
}

impl Monkey {
    pub fn eval(&self, troop: &MonkeyTroop) -> i64 {
        match self {
            Self::Value(v) => *v,
            Self::Oper(m1, s, m2) => s.eval(troop.eval_monkey_at(m1.as_str()), troop.eval_monkey_at(m2.as_str())),
        }
    }

    pub fn names(&self) -> Option<(String,String)> {
        match self {
            Self::Value(_) => None,
            Self::Oper(m1, _, m2) => Some((m1.clone(), m2.clone()))
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