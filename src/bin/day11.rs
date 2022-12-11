use std::str::FromStr;

use advent_code_lib::{all_lines, simpler_main};
use anyhow::bail;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let mut monkeys = vec![];
        let mut lines = all_lines(filename)?;
        loop {
            if let Some(monkey) = Monkey::from_lines(&mut lines) {
                monkeys.push(monkey);
            } else {
                break;
            }
        }
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {i}");
            println!("{monkey:?}");
        }
        Ok(())
    })
}

#[derive(Debug)]
pub enum OpCode {
    Plus, Times
}

impl FromStr for OpCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            "*" => Ok(Self::Times),
            _ => bail!("{s}: Not supported")
        }
    }
}

#[derive(Debug)]
pub struct Operation {
    left: Option<i64>,
    right: Option<i64>,
    op: OpCode,
}

impl Operation {
    pub fn from(s: &str) -> Self {
        println!("s: {s}");
        let mut parts = s.split_whitespace();
        assert_eq!(parts.next().unwrap(), "Operation:");
        assert_eq!(parts.next().unwrap(), "new");
        assert_eq!(parts.next().unwrap(), "=");
        let left = parts.next().unwrap().parse::<i64>().ok();
        let op = parts.next().unwrap().parse::<OpCode>().unwrap();
        let right = parts.next().unwrap().parse::<i64>().ok();
        Operation {left, right, op}
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<i64>,
    op: Operation,
    div_test_value: i64,
    true_monkey: usize,
    false_monkey: usize,
}

fn keep_only<F:Fn(char)->bool>(check: F, s: String) -> String {
    s.chars().map(|c| if check(c) {c} else {' '}).collect()
}

fn keep_digits(s: String) -> String {
    keep_only(|c| c.is_digit(10), s)
}

impl Monkey {
    pub fn from_lines<I: Iterator<Item=String>>(lines: &mut I) -> Option<Self> {
        let line1 = lines.next();
        if line1.is_some() {
            let items = keep_digits(lines.next().unwrap()).split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
            println!("items: {items:?}");
            let op = Operation::from(lines.next().unwrap().as_str());
            let div_test_value = keep_digits(lines.next().unwrap()).split_whitespace().next().unwrap().parse::<i64>().unwrap();
            let true_monkey = keep_digits(lines.next().unwrap()).split_whitespace().next().unwrap().parse::<usize>().unwrap();
            let false_monkey = keep_digits(lines.next().unwrap()).split_whitespace().next().unwrap().parse::<usize>().unwrap();
            lines.next();
            Some(Self {items, op, div_test_value, true_monkey, false_monkey})
        } else {
            None
        }
    }
}