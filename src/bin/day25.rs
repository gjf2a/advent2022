use std::{collections::VecDeque, fmt::Display, str::FromStr};

use advent_code_lib::{all_lines, simpler_main};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        Ok(())
    })
}

fn part1(filename: &str) -> anyhow::Result<Snafu> {
    let mut result = Snafu(0);
    for line in all_lines(filename)? {
        result += line.parse::<Snafu>().unwrap();
    }
    Ok(result)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Snafu(i64);

impl std::ops::AddAssign for Snafu {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl FromStr for Snafu {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut total = 0;
        for c in s.chars() {
            total *= 5;
            total += match c {
                '=' => -2,
                '-' => -1,
                d => (d as u8 - '0' as u8) as i64,
            };
        }
        Ok(Self(total))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut digits = VecDeque::new();
        let mut value = self.0;
        while value > 0 {
            let digit = value % 5;
            value /= 5;
            digits.push_front(if digit < 3 {
                format!("{digit}")
            } else {
                value += 1;
                (if digit == 3 { "=" } else { "-" }).to_owned()
            });
        }
        let result = digits.iter().cloned().collect::<String>();
        write!(f, "{result}")
    }
}
