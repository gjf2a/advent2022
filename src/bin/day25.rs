use std::{str::FromStr, fmt::Display, collections::VecDeque};

use advent_code_lib::{simpler_main, all_lines};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        Ok(())
    })
}

fn part1(filename: &str) -> anyhow::Result<Snafu> {
    let mut result = Snafu(0);
    for line in all_lines(filename)? {
        let s = line.parse::<Snafu>().unwrap();
        snafu_checker(s);
        result += s;
    }
    snafu_checker(result);
    Ok(result)
}

fn snafu_checker(snafu: Snafu) {
    assert_eq!(format!("{snafu}").parse::<Snafu>().unwrap(), snafu);
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
                (if digit == 3 {"="} else {"-"}).to_owned()
            });
        }
        let result = digits.iter().cloned().collect::<String>();
        write!(f, "{result}")
    }
}

/*

New algorithm:

digit = v % 5
v = v / 5
match digit
0, 1, 2 => output same
3 => output =; v += 1
4 => output -; v += 1

4
digit = 4
v = 0
output -; v = 1
output 1
1-

44
digit = 4
v = 8
output -; v = 9
digit = 4
v = 1
output -; v = 2
output 2 
2--

444
digit = 4
v = 88
output -; v = 89
digit = 4
v = 17
output -; v = 18
digit = 3
v = 3
output =; v = 4
digit = 4
v = 0
output -; v = 1
digit = 1
v = 0
output 1
1-=--

625 - 125 - 50 - 5 - 1 = 444

*/