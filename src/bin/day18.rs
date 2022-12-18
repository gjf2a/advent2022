use std::collections::HashMap;

use advent_code_lib::{all_lines, simpler_main, Point};
use enum_iterator::{Sequence, all};

const CUBE_DIM: usize = 3;

type Cubito = Point<i64, CUBE_DIM>;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let droplet = Droplet::from_file(filename)?;
        println!("Part 1: {}", part1(&droplet));
        println!("Part 2: {}", part2(&droplet));
        Ok(())
    })
}

pub fn part1(droplet: &Droplet) -> usize {
    droplet.surface_area(Some(1))
}

pub fn part2(droplet: &Droplet) -> usize {
    droplet.surface_area(None)
}

pub struct Droplet {
    cubes: Vec<Cubito>,
}

impl Droplet {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut cubes = vec![];
        for line in all_lines(filename)? {
            cubes.push(line.parse()?);
        }
        Ok(Self { cubes })
    }

    pub fn surface_area(&self, max_distance: Option<i64>) -> usize {
        let mut result = 0;
        for cube in self.cubes.iter() {
            let neighbors = self.nearest_neighbors(cube, max_distance);
            let in_contact = neighbors.count();
            assert!(in_contact <= 6);
            result += 6 - in_contact;
        }
        result
    }

    pub fn nearest_neighbors(&self, cube: &Cubito, max_distance: Option<i64>) -> NeighborsOf {
        let mut neighbors = NeighborsOf::new(*cube);
        for other in self.cubes.iter() {
            neighbors.add_neighbor(other, max_distance);
        }
        neighbors
    }
}

#[derive(Sequence, Debug, Copy, Clone, Hash,PartialEq, Eq)]
pub enum Dir {
    X, Y, Z
}

impl Dir {
    pub fn dim(&self) -> usize {
        match self {
            Dir::X => 0,
            Dir::Y => 1,
            Dir::Z => 2,
        }
    }
}

#[derive(Sequence, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Pol {
    Pos, Neg
}

impl Pol {
    pub fn new(n: i64) -> Self {
        if n < 0 {
            Pol::Neg
        } else if n > 0 {
            Pol::Pos
        } else {
            panic!("No zero allowed!")
        }
    }
}

pub struct DirPol {
    dir: Dir,
    pol: Pol,
    distance: i64
}

impl DirPol {
    pub fn between(a: &Cubito, b: &Cubito) -> Option<Self> {
        let diff = *a - *b;
        for dir in all::<Dir>() {
            let d = dir.dim();
            if diff[d] != 0 && (0..CUBE_DIM).filter(|i| *i != d && diff[*i] == 0).count() == CUBE_DIM - 1 {
                let pol = Pol::new(diff[d]);
                return Some(DirPol {dir, pol, distance: diff[d].abs()});
            }
        }
        None
    }
}

pub struct NeighborsOf {
    cube: Cubito,
    neighbors: HashMap<(Dir, Pol), (i64, Cubito)>,
}

impl NeighborsOf {
    pub fn new(cube: Cubito) -> Self {
        Self {cube, neighbors: HashMap::new()}
    }

    pub fn count(&self) -> usize {
        self.neighbors.len()
    }

    pub fn add_neighbor(&mut self, neighbor: &Cubito, max_distance: Option<i64>) {
        if let Some(dirpol) = DirPol::between(&self.cube, neighbor) {
            assert_ne!(self.cube, *neighbor);
            if max_distance.map_or(true, |m| dirpol.distance <= m) {
                let key = (dirpol.dir, dirpol.pol);
                match self.neighbors.get_mut(&key) {
                    None => {
                        self.neighbors.insert(key, (dirpol.distance, *neighbor));
                    }
                    Some((distance, prev)) => {
                        if dirpol.distance < *distance {
                            *distance = dirpol.distance;
                            *prev = *neighbor;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Droplet, part1};

    #[test]
    fn test_1() {
        let droplet = Droplet::from_file("ex/day18.txt").unwrap();
        assert_eq!(part1(&droplet), 64);
    }
}
