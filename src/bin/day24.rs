use std::{collections::BTreeMap, fmt::Display, cmp::max};

use advent_code_lib::{all_lines, simpler_main, ManhattanDir, Point};
use enum_iterator::all;

type Pt = Point<isize,2>;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        println!("Part 2: {}", part2(filename)?);
        Ok(())
    })
}

fn part1(filename: &str) -> anyhow::Result<usize> {
    let map = BlizzardMap::from_file(filename)?;
    println!("{map}");
    Ok(0)
}

fn part2(filename: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum BlizzardCell {
    Wall, Wind(Wind)
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Default)]
struct Wind {
    has_wind: [bool; 4]
}

impl Wind {
    fn dir(dir: ManhattanDir) -> Self {
        let mut result = Self::default();
        result.has_wind[dir as usize] = true;
        result
    }
}

impl Display for Wind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let winds = self.winds();
        let count_str = format!("{}", winds.len());
        let s = match winds.len() {
            0 => ".",
            1 => match winds[0] {
                ManhattanDir::N => "^",
                ManhattanDir::E => ">",
                ManhattanDir::S => "v",
                ManhattanDir::W => "<",
            }
            _ => count_str.as_str()
        };
        write!(f, "{s}")
    }
}

impl Wind {
    fn is_windy(&self) -> bool {
        self.has_wind.iter().any(|w| *w)
    }

    fn winds(&self) -> Vec<ManhattanDir> {
        all::<ManhattanDir>().enumerate().filter(|(i, _)| self.has_wind[*i]).map(|(_,d)| d).collect()
    }
}

impl Display for BlizzardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlizzardCell::Wall => write!(f, "#"),
            BlizzardCell::Wind(w) => write!(f, "{}", *w)
        }
    }
}

impl From<char> for BlizzardCell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Wind(Wind::default()),
            '#' => Self::Wall,
            '^' => Self::Wind(Wind::dir(ManhattanDir::N)),
            '>' => Self::Wind(Wind::dir(ManhattanDir::E)),
            'v' => Self::Wind(Wind::dir(ManhattanDir::S)),
            '<' => Self::Wind(Wind::dir(ManhattanDir::W)),
            _ => panic!("Illegal character"), 
        }
    }
}

#[derive(Clone, Debug)]
struct BlizzardMap {
    wind_map: BTreeMap<Pt,BlizzardCell>,
    width: isize,
    height: isize,
}

impl BlizzardMap {
    fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut wind_map = BTreeMap::new();
        let mut height = 0;
        let mut width = 0;
        for (row, line) in all_lines(filename)?.enumerate() {
            for (col, c) in line.chars().enumerate() {
                width = max(width, col as isize + 1);
                wind_map.insert(Pt::new([col as isize, row as isize]), c.into());
            }
            height += 1;
        }
        Ok(Self {wind_map, width, height})
    }
}

impl Display for BlizzardMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.wind_map.get(&Pt::new([col, row])).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}