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
                let line_list = line.parse::<List>()?;
                assert_eq!(format!("{}", line_list), line);
                pair.push(line_list);
            } else {
                pairs.push(pair);
                pair = vec![];
            }
        }
        pairs.push(pair);
        for pair in pairs.iter() {
            println!("{}", pair[0]);
            println!("{}", pair[1]);
            println!();
        }
        Ok(())
    })
}

pub enum List {
    Value(i64),
    Values(Vec<List>),
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
