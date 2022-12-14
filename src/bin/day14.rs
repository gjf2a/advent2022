use std::cmp::{min, max};

use advent_code_lib::{simpler_main, InfiniteGrid, all_lines};


fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let rocks = RockSection::from_file(filename)?;
        println!("{rocks:?}");
        Ok(())
    })
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Contents {
    #[default]
    Air, 
    Sand, 
    Rock,
}

#[derive(Default, Clone, Debug)]
pub struct RockSection {
    cells: InfiniteGrid<Contents>
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
}