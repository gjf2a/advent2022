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
        print!("line: {line}");
        let s = line.parse::<Snafu>().unwrap();
        println!(" {}", s.0);
        snafu_checker(s);
        result += s;
    }
    println!("(place, digit): {:?}", ascend_place_digit(result.0));
    snafu_checker(result);
    Ok(result)
}

fn snafu_checker(snafu: Snafu) {
    assert_eq!(format!("{snafu}").parse::<Snafu>().unwrap(), snafu);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

fn ascend_place_digit(value: isize) -> (u32,isize) {
    let mut opt1 = 1;
    let mut opt2 = 2;
    let mut place = 0;
    loop {
        if opt1 >= value {
            return (place, 1);
        } else if opt2 >= value {
            return (place, 2);
        }
        opt1 *= 5;
        opt2 *= 5;
        place += 1;
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut place, digit) = ascend_place_digit(self.0);
        write!(f, "{digit}")?;
        let mut value = digit * 5_isize.pow(place);
        while place > 0 {
            place -= 1;
            let adjust = 5_isize.pow(place);
            let mut digit = 0;
            if value < self.0 {
                value += adjust;
                digit += 1;
                if value < self.0 {
                    value += adjust;
                    digit += 1;
                }
            } else if value > self.0 {
                value -= adjust;
                digit -= 1;
                if value > self.0 {
                    value -= adjust;
                    digit -= 1;
                }
            }
            match digit {
                -2 => write!(f, "=")?,
                -1 => write!(f, "-")?,
                0..=2 => write!(f, "{digit}")?,
                err => panic!("Erroneous digit: {err}"),    
            }
        }
        Ok(())
    }
}

/*
4890

2 * 3125 = 6250
"2"
6250 - 625 = 5625
5625 - 625 = 5000
"2="
5000 - 125 = 4875
"2=-"
4875 + 25 = 4900
"2=-1"
4900 - 5 = 4895
4895 - 5 = 4890
"2=-1="
"2=-1=0"
*/