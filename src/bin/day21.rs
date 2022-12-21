use std::{collections::{BTreeMap, BTreeSet, VecDeque}, str::FromStr};

use advent_code_lib::{all_lines, simpler_main};
use anyhow::bail;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        println!("Part 2: {}", part2(filename)?);
        Ok(())
    })
}

pub fn part1(filename: &str) -> anyhow::Result<i64> {
    let troop = MonkeyTroop::from_file(filename)?;
    Ok(troop.evaluate_root())
}

pub fn part2(filename: &str) -> anyhow::Result<i64> {
    let troop = MonkeyTroop::from_file(filename)?;
    Ok(troop.find_human_yell())
}

#[derive(Clone)]
pub struct MonkeyTroop {
    monkeys: BTreeMap<String,Monkey>,
    root_name: String,
    term2lefts: BTreeMap<String,Vec<String>>,
}

fn add_left_of(on_left_of: &mut BTreeMap<String,Vec<String>>, right: &str, left: &str) {
    match on_left_of.get_mut(left) {
        Some(v) => {v.push(right.to_owned());}
        None => {on_left_of.insert(left.to_owned(), vec![right.to_owned()]);}
    }
}

impl MonkeyTroop {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut monkeys = BTreeMap::new();
        let mut term2lefts = BTreeMap::new();
        for line in all_lines(filename)? {
            let mut parts = line.split_whitespace();
            let name = parts.next().unwrap();
            let name = &name[..4];
            let formula: Vec<String> = parts.map(|p| p.to_string()).collect();
            let monkey = match formula.len() {
                1 => Monkey::Value(formula[0].parse::<i64>()?),
                3 => {
                    add_left_of(&mut term2lefts, name, formula[0].as_str());
                    add_left_of(&mut term2lefts, name, formula[2].as_str());
                    Monkey::Oper(formula[0].clone(), formula[1].parse::<Sym>()?, formula[2].clone())
                }
                err => panic!("formula has {err} terms; not allowed")
            };
            monkeys.insert(name.to_owned(), monkey);
        }
        Ok(Self {root_name: "root".to_owned(), monkeys, term2lefts})
    }

    fn root_monkey(&self) -> Monkey {
        self.monkeys.get(self.root_name.as_str()).unwrap().clone()
    }

    pub fn find_human_yell(&self) -> i64 {
        let (t1, t2) = self.split_troop();
        let (human_troop, other_troop) = if t1.has_human() {(&t1, &t2)} else {(&t2, &t1)};
        let goal = other_troop.evaluate_root();
        human_troop.solve_human(goal)
    }

    fn reachable_names(&self) -> BTreeSet<String> {
        let mut result = BTreeSet::new();
        self.reachable_help(self.root_name.as_str(), &mut result);
        result
    }

    fn reachable_help(&self, name: &str, reachable_names: &mut BTreeSet<String>) {
        reachable_names.insert(name.to_owned());
        match self.monkeys.get(name).unwrap() {
            Monkey::Value(_) => {}
            Monkey::Oper(m1, _, m2) => {
                self.reachable_help(m1.as_str(), reachable_names);
                self.reachable_help(m2.as_str(), reachable_names);
            }
        }
    }

    pub fn has_human(&self) -> bool {
        self.reachable_names().contains("humn")
    }

    fn solve_human(&self, goal: i64) -> i64 {
        let mut unknown = self.reachable_names();
        let mut known = BTreeMap::new();
        known.insert(self.root_name.clone(), goal);
        for name in unknown.iter() {
            if let Monkey::Value(v) = self.monkeys.get(name).unwrap() {
                if name != "humn" {
                    known.insert(name.clone(), *v);
                }
            }
        }

        for name in known.keys() {
            unknown.remove(name);
        }

        let mut unknown: VecDeque<String> = unknown.iter().cloned().collect();
        while let Some(candidate) = unknown.pop_front() {
            let candidate_monkey = self.monkeys.get(candidate.as_str()).unwrap();
            if let Some((candidate_left, candidate_right)) = candidate_monkey.names() {
                if known.contains_key(candidate_left.as_str()) && known.contains_key(candidate_right.as_str()) {
                    let value = candidate_monkey.eval(self);
                    known.insert(candidate.clone(), value);
                }
            }
            if !known.contains_key(candidate.as_str()) {
                let lhs = self.term2lefts.get(candidate.as_str()).unwrap();
                assert_eq!(lhs.len(), 1);
                let left_monkey = self.monkeys.get(lhs[0].as_str()).unwrap();
                let (op_left, op_right) = left_monkey.names().unwrap();
                assert!(op_left == candidate || op_right == candidate);
                if op_left == candidate && known.contains_key(op_right.as_str()) && known.contains_key(lhs[0].as_str()) {
                    let value = left_monkey.sym().unwrap().solve_left(*known.get(lhs[0].as_str()).unwrap(), *known.get(op_right.as_str()).unwrap());
                    known.insert(candidate, value);
                } else if op_right == candidate && known.contains_key(op_left.as_str()) && known.contains_key(lhs[0].as_str()) {
                    let value = left_monkey.sym().unwrap().solve_right(*known.get(lhs[0].as_str()).unwrap(), *known.get(op_left.as_str()).unwrap());
                    known.insert(candidate, value);
                } else {
                    unknown.push_back(candidate);
                }
            }
        }
        *known.get("humn").unwrap()
    }

    pub fn split_troop(&self) -> (Self, Self) {
        let mut left = self.clone();
        let mut right = self.clone();
        let root_monkey = self.root_monkey();
        let (left_name, right_name) = root_monkey.names().unwrap();
        left.root_name = left_name;
        right.root_name = right_name;
        (left, right)
    }

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

    pub fn sym(&self) -> Option<Sym> {
        match self {
            Self::Value(_) => None,
            Self::Oper(_, s, _) => Some(*s)
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

    pub fn solve_left(&self, result: i64, right: i64) -> i64 {
        match self {
            Sym::Plus => result - right,
            Sym::Minus => result + right,
            Sym::Times => result / right,
            Sym::Divide => result * right,
        }
    }

    pub fn solve_right(&self, result: i64, left: i64) -> i64 {
        match self {
            Sym::Plus => result - left,
            Sym::Minus => left - result,
            Sym::Times => result / left,
            Sym::Divide => left / result,
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

/* 
Part 2 notes 

Relevant lines:

ptdq: humn - dvpt
dvpt: 3
lgvd: ljgn * ptdq
cczh: sllz + lgvd
pppw: cczh / lfqf
lfqf: 4
ljgn: 2

pppw is at root

humn = ptdq + dvpt (3)
ptdq = lgvd / ljgn (2)
lgvd = cczh - sllz (4)
cczh = pppw (150) * lfqf (4)

150 * 4 = 600
600 - 4 = 596
596 / 2 = 298
298 + 3 = 301
*/