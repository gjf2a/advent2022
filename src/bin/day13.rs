use std::cmp::{min, Ordering};
use std::fmt::Display;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

use advent_code_lib::{all_lines, simpler_main};
use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let mut pairs = vec![];
        let mut pair = vec![];
        for line in all_lines(filename)? {
            if line.len() > 0 {
                let line_list = line.parse()?;
                assert_eq!(format!("{}", line_list), line);
                pair.push(line_list);
            } else {
                pairs.push(pair);
                pair = vec![];
            }
        }
        pairs.push(pair);
        println!("Part 1: {}", part1(&pairs));
        println!("Part 2: {}", part2(&pairs));
        Ok(())
    })
}

pub fn part1(pairs: &Vec<Vec<List>>) -> usize {
    let mut index_total = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair[0] < pair[1] {
            index_total += i + 1;
        }
    }
    index_total
}

pub fn part2(pairs: &Vec<Vec<List>>) -> usize {
    let mut flattened = vec![];
    for pair in pairs.iter() {
        for item in pair.iter() {
            flattened.push(item.clone());
        }
    }
    let dividers: Vec<List> = ["[[2]]", "[[6]]"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    dividers
        .iter()
        .for_each(|divider| flattened.push(divider.clone()));
    flattened.sort();
    dividers
        .iter()
        .map(|divider| divider_index_in(&flattened, divider))
        .product()
}

pub fn divider_index_in(sorted: &Vec<List>, divider: &List) -> usize {
    sorted
        .iter()
        .enumerate()
        .find(|(_, v)| **v == *divider)
        .map(|(i, _)| i)
        .unwrap()
        + 1
}

#[derive(Eq, PartialEq, Ord, Debug, Clone)]
pub enum List {
    Value(i64),
    Values(Vec<List>),
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some((i1, i2)) = self
            .integer()
            .and_then(|i1| other.integer().map(|i2| (i1, i2)))
        {
            i1.partial_cmp(&i2)
        } else {
            let l1 = self.list();
            let l2 = other.list();
            let len1 = l1.len();
            let len2 = l2.len();
            for i in 0..min(len1, len2) {
                match l1[i].partial_cmp(&l2[i]).unwrap() {
                    Ordering::Equal => {}
                    ordering => return Some(ordering),
                }
            }
            len1.partial_cmp(&len2)
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", *v),
            Self::Values(vs) => {
                let mut vs = vs.iter();
                let first = match vs.next() {
                    None => "".to_owned(),
                    Some(v) => format!("{}", *v),
                };
                write!(f, "[{}", first)?;
                loop {
                    match vs.next() {
                        None => return write!(f, "]"),
                        Some(v) => write!(f, ",{}", *v)?,
                    }
                }
            }
        }
    }
}

impl List {
    pub fn integer(&self) -> Option<i64> {
        if let Self::Value(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn list(&self) -> Vec<Self> {
        match self {
            List::Value(v) => vec![List::Value(*v)],
            List::Values(vs) => vs.clone(),
        }
    }

    fn recursive_parse(chars: &mut Peekable<Chars>) -> anyhow::Result<Self> {
        let test = chars.peek().ok_or(anyhow!("No input"))?;
        if test.is_digit(10) {
            Self::parse_number(chars)
        } else if *test == '[' {
            Self::parse_list(chars)
        } else {
            bail!("Unrecognized character: {test}")
        }
    }

    fn parse_number(chars: &mut Peekable<Chars>) -> anyhow::Result<Self> {
        let mut number = String::new();
        number.push(chars.next().unwrap());
        loop {
            match chars.peek() {
                None => return Ok(Self::Value(number.parse()?)),
                Some(digit) => {
                    if digit.is_digit(10) {
                        number.push(chars.next().unwrap());
                    } else {
                        return Ok(Self::Value(number.parse()?));
                    }
                }
            }
        }
    }

    fn parse_list(chars: &mut Peekable<Chars>) -> anyhow::Result<Self> {
        chars.next();
        let mut list = vec![];
        loop {
            match chars.peek().ok_or(anyhow!("Unmatched '['"))? {
                ']' => {
                    chars.next();
                    return Ok(Self::Values(list));
                }
                ',' => {
                    chars.next();
                }
                _ => {
                    list.push(Self::recursive_parse(chars)?);
                }
            }
        }
    }
}

impl FromStr for List {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut chars = s.chars().peekable();
        Self::recursive_parse(&mut chars)
    }
}
