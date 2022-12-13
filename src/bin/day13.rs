use std::fmt::Display;
use std::str::{FromStr, Chars};
use std::iter::Peekable;

use advent_code_lib::{all_lines, simpler_main};
use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        for line in all_lines(filename)? {
            if line.len() > 0 {
                let line_list = line.parse::<List>()?;
                println!("{line_list}");
            }
        }
        Ok(())
    })
}

pub enum List {
    Value(i64),
    Values(Vec<List>)
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", *v),
            Self::Values(vs) => {
                write!(f, "[")?;
                for v in vs {
                    write!(f, "{},", *v)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl List {
    fn recursive_parse(chars: &mut Peekable<Chars>) -> anyhow::Result<Self> {
        let test = chars.peek().ok_or(anyhow!("No input"))?;
        if test.is_digit(10) {
            let mut number = String::new();
            number.push(chars.next().unwrap());
            loop {
                match chars.peek() {
                    None => return Ok(Self::Value(number.parse().unwrap())),
                    Some(digit) => {
                        if digit.is_digit(10) {
                            number.push(chars.next().unwrap());
                        } else {
                            return Ok(Self::Value(number.parse().unwrap()));
                        }
                    }
                }
            }
        } else if *test == '[' {
            chars.next();
            let mut list = vec![];
            loop {
                match chars.peek().ok_or(anyhow!("Unmatched '['"))? {
                    ']' => return Ok(Self::Values(list)),
                    ',' => {
                        chars.next();
                    }
                    _ => {
                        list.push(Self::recursive_parse(chars)?);
                    }
                }
            }
        } else {
            bail!("Unrecognized character: {test}")
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