use std::{collections::{BTreeSet, BTreeMap}, fmt::Display};

use advent_code_lib::{all_lines, simpler_main, ManhattanDir, Point, Dir};
use bare_metal_modulo::{MNum, ModNumC};

type Elf = Point<isize, 2>;

const ORDERING: [ManhattanDir; 4] = [
    ManhattanDir::N,
    ManhattanDir::S,
    ManhattanDir::W,
    ManhattanDir::E,
];
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
    println!("{elves:?}");
    println!("{elves}");
    for r in 0..10 {
        elves.round();
        println!("After round {}:", r + 1);
        println!("{elves}");
    }
    Ok(elves.empty_space())
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CellularElves {
    elves: BTreeSet<Elf>,
    dir_start: ModNumC<usize, ORDERING_LEN>,
}

impl CellularElves {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut elves = BTreeSet::new();
        for (row, line) in all_lines(filename)?.enumerate() {
            for (col, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
                elves.insert(Elf::new([col as isize, row as isize]));
            }
        }
        Ok(Self {
            elves,
            dir_start: ModNumC::new(0),
        })
    }

    pub fn elf(&self, col: isize, row: isize) -> bool {
        self.elves.contains(&Elf::new([col, row]))
    }

    pub fn min_elf_rectangle_pts(&self) -> Vec<(isize, isize)> {
        let mut result = vec![];
        let (min_elf, max_elf) = Elf::min_max_points(self.elves.iter().copied()).unwrap();
        for row in min_elf[1]..=max_elf[1] {
            for col in min_elf[0]..=max_elf[0] {
                result.push((col, row));
            }
        }
        result
    }

    pub fn round(&mut self) {
        let starting_elves = self.elves.len();
        let proposals = self.proposed_moves();
        println!("proposals: {proposals:?}");
        let moves = proposals.iter().filter(|(_,ps)| ps.len() == 1).map(|(end, start)| (*end, start[0]));
        println!("movables:  {:?}", moves.clone().collect::<Vec<_>>());
        for (end, start) in proposals.iter().filter(|(_,ps)| ps.len() == 1).map(|(end, start)| (*end, start[0])) {
            self.elves.remove(&start);
            self.elves.insert(end);
        }
        assert_eq!(starting_elves, self.elves.len());
        self.dir_start += 1;
    }

    pub fn proposed_moves(&self) -> BTreeMap<Elf,Vec<Elf>> {
        let mut end2starts = BTreeMap::new();
        for elf in self.elves.iter() {
            if let Some(proposal) = self.proposed_move(*elf) {
                match end2starts.get_mut(&proposal) {
                    None => {end2starts.insert(proposal, vec![*elf]);},
                    Some(v) => {v.push(*elf);},
                }
            }
        }
        end2starts
    }

    fn proposed_move(&self, elf: Elf) -> Option<Elf> {
        self.dir_start.iter().find_map(|i| self.proposal_for(elf, ORDERING[i.a()]))
    }

    fn proposal_for(&self, elf: Elf, dir: ManhattanDir) -> Option<Elf> {
        if dir_check(dir).iter().all(|check| !self.elves.contains(&elf.dir_moved(*check))) {
            Some(elf.manhattan_moved(dir))
        } else {
            None
        }
    }

    pub fn empty_space(&self) -> usize {
        self.min_elf_rectangle_pts()
            .iter()
            .filter(|(col, row)| !self.elf(*col, *row))
            .count()
    }
}

fn dir_check(dir: ManhattanDir) -> [Dir; 3] {
    match dir {
        ManhattanDir::N => [Dir::Ne, Dir::N, Dir::Nw],
        ManhattanDir::E => [Dir::Ne, Dir::E, Dir::Se],
        ManhattanDir::S => [Dir::Se, Dir::S, Dir::Sw],
        ManhattanDir::W => [Dir::Nw, Dir::W, Dir::Sw],
    }
}

impl Display for CellularElves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_elf, max_elf) = Elf::min_max_points(self.elves.iter().copied()).unwrap();
        for row in min_elf[1]..=max_elf[1] {
            for col in min_elf[0]..=max_elf[0] {
                let c = if self.elf(col, row) { '#' } else { '.' };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
