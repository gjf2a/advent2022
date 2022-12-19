use std::{cmp::min, collections::BTreeSet};

use advent_code_lib::{all_lines, heuristic_search, simpler_main, Point};

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
    droplet.surface_area(|_| true)
}

pub fn part2(droplet: &Droplet) -> usize {
    droplet.surface_area(|cube| !droplet.trapped(cube))
}

pub struct Droplet {
    cubes: BTreeSet<Cubito>,
}

impl Droplet {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        Ok(Self {
            cubes: all_lines(filename)?.map(|s| s.parse().unwrap()).collect(),
        })
    }

    pub fn surface_area<I: Fn(&Cubito) -> bool>(&self, include: I) -> usize {
        let mut result = 0;
        for cube in self.cubes.iter() {
            for neighbor in cube.manhattan_neighbors() {
                if !self.cubes.contains(&neighbor) && include(&neighbor) {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn trapped(&self, cube: &Cubito) -> bool {
        let (min_p, max_p) = Cubito::min_max_points(self.cubes.iter().copied()).unwrap();
        let at_goal = |n: &Cubito| {
            n.values()
                .enumerate()
                .any(|(i, v)| v < min_p[i] || v > max_p[i])
        };
        let heuristic = |n: &Cubito| {
            (0..3)
                .map(|i| min((n[i] - min_p[i]).abs(), (n[i] - max_p[i]).abs()))
                .min()
                .unwrap()
        };
        let get_successors = |n: &Cubito| {
            n.manhattan_neighbors()
                .iter()
                .copied()
                .filter(|n| !self.cubes.contains(n))
                .collect()
        };
        let result = heuristic_search(*cube, at_goal, heuristic, get_successors);
        !at_goal(&result.path().unwrap().back().copied().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, Cubito, Droplet};

    #[test]
    fn test_1() {
        let droplet = Droplet::from_file("ex/day18.txt").unwrap();
        assert_eq!(part1(&droplet), 64);
    }

    #[test]
    fn test_trapped() {
        let droplet = Droplet::from_file("ex/day18.txt").unwrap();
        assert!(droplet.trapped(&Cubito::new([2, 2, 5])));
        assert!(!droplet.trapped(&Cubito::new([0, 2, 2])))
    }
}
