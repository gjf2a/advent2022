use std::{str::FromStr, fmt::Display};

use advent_code_lib::{simpler_main, all_lines};



fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        //println!("Part 2: {}", part2(filename)?);
        Ok(())
    })
}

fn part1(filename: &str) -> anyhow::Result<Snafu> {
    let mut result = Snafu(0);
    for line in all_lines(filename)? {
        result += line.parse().unwrap();
    }
    Ok(result)
}

struct Snafu(isize);

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
                d => (d as u8 - '0' as u8) as isize,
            };
        }
        Ok(Self(total))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}