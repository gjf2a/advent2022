use std::{collections::BTreeMap, cmp::{max, min}, str::FromStr, ops::RangeInclusive, fmt::Display};

use advent_code_lib::{all_lines, simpler_main, Point, ManhattanDir};
use anyhow::bail;

type Pt = Point<isize,2>;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let (map, path) = map_path_from(filename)?;
        println!("{map}");
        println!("{path}");
        Ok(())
    })
}

pub fn map_path_from(filename: &str) -> anyhow::Result<(Map, Path)> {
    let mut lines = all_lines(filename)?;
    let mut map_lines = vec![];
    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }
        map_lines.push(line);
    }
    let instruction_line = lines.next().unwrap();
    assert!(lines.next().is_none());
    Ok((Map::from_lines(map_lines)?, instruction_line.parse()?))
}

#[derive(Debug, Clone)]
pub struct Map {
    map: BTreeMap<Pt, MapCell>,
    row2cols: Vec<RangeInclusive<isize>>,
    col2rows: Vec<RangeInclusive<isize>>,
}

impl Map {
    pub fn from_lines(lines: Vec<String>) -> anyhow::Result<Self> {
        let mut map = BTreeMap::new();
        let mut longest_line_len = 0;
        for (row, line) in lines.iter().enumerate() {
            longest_line_len = max(longest_line_len, line.len());
            for (col, c) in line.chars().enumerate() {
                if ['#', '.'].contains(&c) {
                    let p = Pt::new([col as isize, row as isize]);
                    let cell = if c == '#' {MapCell::Wall} else {MapCell::Space};
                    map.insert(p, cell);
                } else if c != ' ' {
                    bail!("{c} is illegal")
                }
            }
        }
        let row2cols = extract_ranges_from(&map, lines.len() as isize, longest_line_len as isize, 0, |outer, inner| [inner, outer]);
        let col2rows = extract_ranges_from(&map, longest_line_len as isize, lines.len() as isize, 1, |outer, inner| [outer, inner]);
        Ok(Map {map, row2cols, col2rows})
    }
}

fn extract_ranges_from<F:Fn(isize,isize)->[isize;2]>(map: &BTreeMap<Pt,MapCell>, outer: isize, inner: isize, index: usize, indexer: F) -> Vec<RangeInclusive<isize>> {
    let mut result = vec![];
    for i in 0..outer {
        let mut min_v = inner;
        let mut max_v = 0;
        for j in 0..inner {
            let p = Pt::new(indexer(i, j));
            if map.contains_key(&p) {
                min_v = min(min_v, p[index]);
                max_v = max(max_v, p[index]);
            }
        }
        result.push(min_v..=max_v);
    }
    result
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row, col_range) in self.row2cols.iter().enumerate() {
            for _ in 0..*col_range.start() {
                write!(f, " ")?;
            }
            for col in col_range.clone() {
                let p = Pt::new([col, row as isize]);
                write!(f, "{}", self.map.get(&p).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MapCell {
    Wall, Space
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {Self::Wall => '#', Self::Space => '.'})
    }
}

pub struct Path {
    path: Vec<PathMove>
}

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(['R', 'L']).map(|ns| ns.parse::<PathMove>().unwrap());
        let mut moves = s.split(|c: char| c.is_digit(10)).filter(|ms| ms.len() > 0).map(|ms| ms.parse::<PathMove>().unwrap());
        let mut path = vec![];
        while let Some(num) = nums.next() {
            path.push(num);
            if let Some(m) = moves.next() {
                path.push(m);
            }
        }
        Ok(Path{path})
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.path.iter() {
            write!(f, "{m}")?;
        }
        Ok(())
    }
}

pub enum PathMove {
    Forward(isize), Left, Right
}

impl FromStr for PathMove {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => match s.parse::<isize>() {
                Ok(n) => Ok(Self::Forward(n)),
                Err(e) => bail!(e)
            }
        }
    }
}

impl Display for PathMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {Self::Left => "L".to_owned(), Self::Right => "R".to_owned(), Self::Forward(n) => format!("{n}")})
    }
}

pub struct PathPosition {
    position: Pt,
    orientation: ManhattanDir,
}