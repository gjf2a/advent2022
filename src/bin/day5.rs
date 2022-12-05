use std::{io, collections::VecDeque};
use advent_code_lib::{advent_main, all_lines};

fn main() -> io::Result<()> {
    advent_main(&[], &[], |args| { 
        let mut puzzle1 = CratePuzzle::from_file(args[1].as_str()).unwrap();
        let mut puzzle2 = puzzle1.clone();
        puzzle1.run1();
        println!("Part 1: {}", puzzle1.state.tops());
        puzzle2.run2();
        println!("Part 2: {}", puzzle2.state.tops());
        Ok(())
    })
}

#[derive(Clone, Debug)]
pub struct CratePuzzle {
    state: CrateState,
    script: Vec<CrateInstruction>,
}

#[derive(Clone, Debug)]
pub struct CrateState {
    stacks: Vec<VecDeque<char>>
}

impl CrateState {
    pub fn tops(&self) -> String {
        self.stacks.iter().map(|s| s.back().copied().unwrap()).collect()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CrateInstruction {
    pub quantity: usize,
    pub start: usize,
    pub end: usize,
}

impl CrateInstruction {
    fn part1(&self, puzzle: &mut CrateState) {
        let start = self.start - 1;
        let end = self.end - 1;
        for _ in 0..self.quantity {
            let parcel = puzzle.stacks[start].pop_back().unwrap();
            puzzle.stacks[end].push_back(parcel);
        }
    }

    fn part2(&self, puzzle: &mut CrateState) {
        let start = self.start - 1;
        let end = self.end - 1;
        let mut parcels = VecDeque::new();
        for _ in 0..self.quantity {
            let parcel = puzzle.stacks[start].pop_back().unwrap();
            parcels.push_front(parcel);
        }
        while parcels.len() > 0 {
            puzzle.stacks[end].push_back(parcels.pop_front().unwrap());
        }
    }
}

fn decode_char(c: Option<&char>) -> Option<char> {
    match c {
        None => None,
        Some(c) => match *c {
            'A'..='Z' => Some(*c),
            _ => None
        }
    }
}

impl CratePuzzle {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut stacks = vec![];
        let mut state = None;
        let mut script = vec![];
        for line in all_lines(filename)? {
            if line.starts_with("move") {
                let line = line.replace("move", "to").replace("from", "").replace("to", "");
                let nums = line.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
                script.push(CrateInstruction {quantity: nums[0], start: nums[1], end: nums[2]});
            } else if line.starts_with(" 1   2   3") {
                state = Some(CrateState {stacks: stacks.clone()});
            } else if line.trim().len() > 0 {
                let chars = line.chars().collect::<Vec<_>>();
                let num_stacks = (chars.len() + 1) / 4;
                if stacks.len() < num_stacks {
                    for _ in 0..(num_stacks - stacks.len()) {
                        stacks.push(VecDeque::new());
                    } 
                }
                (0..stacks.len())
                    .map(|i| (i, decode_char(chars.get(1 + i * 4))))
                    .for_each(|(i,c)| {
                        if let Some(c) = c {
                            stacks[i].push_front(c);
                        }
                    });
            };
        };
        Ok(Self {state: state.unwrap(), script})
    }

    pub fn run1(&mut self) {
        for line in self.script.iter() {
            line.part1(&mut self.state);
        }
    }

    pub fn run2(&mut self) {
        for line in self.script.iter() {
            line.part2(&mut self.state);
        }
    }
}