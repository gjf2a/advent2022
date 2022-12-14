use std::{cmp::{min, max}, fmt::Display};

use advent_code_lib::{simpler_main, InfiniteGrid, all_lines, Position, Dir};


fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let rocks = RockSection::from_file(filename)?;
        println!("Part 1: {}", part1(rocks.clone()));
        println!("Part 2: {}", part2(rocks));
        Ok(())
    })
}

pub fn part1(mut rocks: RockSection) -> usize {
    rocks.pour_sand_until_full();
    rocks.sand_count
}

pub fn part2(mut rocks: RockSection) -> usize {
    rocks.add_floor();
    rocks.pour_sand_until_full();
    rocks.sand_count
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Contents {
    #[default]
    Air, 
    Sand, 
    Rock,
}

impl Display for Contents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Air => ".",
            Self::Rock => "#",
            Self::Sand => "o",
        })
    }
}

#[derive(Default, Clone, Debug)]
pub struct RockSection {
    cells: InfiniteGrid<Contents>,
    sand_count: usize,
    floor_level: Option<isize>,
}

impl Display for RockSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cells)
    }
}

fn pair_from(s: &str) -> (isize, isize) {
    let mut parts = s.split(",");
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();
    (x, y)
}

impl RockSection {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut result = Self::default();
        for line in all_lines(filename)? {
            result.add_path(line.as_str());
        }
        Ok(result)
    }

    pub fn add_floor(&mut self) {
        self.floor_level = Some(self.cells.max_y() + 2);
    }

    pub fn blocked(&self, p: Position) -> bool {
        self.floor_level.map_or(false, |f| p.row >= f) || self.cells.get_pos(p) != Contents::Air
    }

    pub fn add_path(&mut self, path: &str) {
        let mut pairs = path.split_whitespace().filter(|p| *p != "->");
        let (mut x1, mut y1) = pair_from(pairs.next().unwrap());
        for pair in pairs {
            let (x2, y2) = pair_from(pair);
            if x2 == x1 {
                for y in min(y1, y2)..=max(y1, y2) {
                    self.cells.add(x1, y, Contents::Rock);
                } 
            } else {
                for x in min(x1, x2)..=max(x1, x2) {
                    self.cells.add(x, y1, Contents::Rock);
                }
            }
            x1 = x2;
            y1 = y2;
        }
    }

    pub fn add_sand(&mut self) {
        let mut sand_pos = Position {col: 500, row: 0};
        if self.blocked(sand_pos) {
            return;
        }
        let bottom = match self.floor_level {
            None => Some(self.cells.max_y()),
            Some(_) => None,
        };
        loop {
            match self.next_sand_move(sand_pos) {
                Some(updated) => {
                    if bottom.map_or(false, |bottom| updated.row > bottom) {
                        return;
                    } else {
                        sand_pos = updated;
                    }
                }
                None => {
                    self.cells.add_pos(sand_pos, Contents::Sand);
                    self.sand_count += 1;
                    return;
                }
            }
        }
    }

    pub fn pour_sand_until_full(&mut self) {
        let mut last_count = self.sand_count;
        loop {
            self.add_sand();
            if self.sand_count == last_count {
                return;
            } else {
                last_count = self.sand_count;
            }
        }
    }

    pub fn next_sand_move(&self, sand_pos: Position) -> Option<Position> {
        [Dir::S, Dir::Sw, Dir::Se].iter().map(|d| sand_pos.updated(*d)).find(|p| !self.blocked(*p))
    }
}