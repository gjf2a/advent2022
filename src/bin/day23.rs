use std::{collections::BTreeSet, fmt::Display};

use advent_code_lib::{all_lines, simpler_main, ManhattanDir, Point};
use bare_metal_modulo::{MNum,ModNumC};

type Elf = Point<isize,2>;

const ORDERING: [ManhattanDir; 4] = [ManhattanDir::N, ManhattanDir::S, ManhattanDir::W, ManhattanDir::E];
const ORDERING_LEN: usize = ORDERING.len();

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        //println!("Part 2: {}", part2(filename)?);
        Ok(())
    })
}

fn part1(filename: &str) -> anyhow::Result<usize> {
    let mut elves = CellularElves::from_file(filename)?;
    println!("{elves}");
    Ok(elves.empty_space())
}

pub struct CellularElves {
    elves: BTreeSet<Elf>,
    dir_start: ModNumC<usize,ORDERING_LEN>,
}

impl CellularElves {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut elves = BTreeSet::new();
        for (row, line) in all_lines(filename)?.enumerate() {
            for (col, _) in line.chars().enumerate().filter(|(_,c)| *c == '#') {
                elves.insert(Elf::new([col as isize, row as isize]));
            }
        }
        Ok(Self {elves, dir_start: ModNumC::new(0)})
    }

    pub fn elf(&self, col: isize, row: isize) -> bool {
        self.elves.contains(&Elf::new([col, row]))
    }

    pub fn empty_space(&self) -> usize {
        let mut space = 0;
        let (min_elf, max_elf) = Elf::min_max_points(self.elves.iter().copied()).unwrap();
        for row in min_elf[1]..=max_elf[1] {
            for col in min_elf[0]..=max_elf[0] {
                if self.elf(col, row) {
                    space += 1;
                }
            }
        }
        space
    }
}

impl Display for CellularElves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_elf, max_elf) = Elf::min_max_points(self.elves.iter().copied()).unwrap();
        for row in min_elf[1]..=max_elf[1] {
            for col in min_elf[0]..=max_elf[0] {
                let c = if self.elf(col, row) {'#'} else {'.'};
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}