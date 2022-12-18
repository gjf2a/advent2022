use advent_code_lib::{all_lines, simpler_main, Point};

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
    droplet.surface_area(|a, b| a.adjacent(b))
}

pub fn eventually_adjacent(a: &Cubito, b: &Cubito) -> bool {
    (0..CUBE_DIM).filter(|i| a.coords[*i] == b.coords[*i]).count() == CUBE_DIM - 1
}

pub fn part2(droplet: &Droplet) -> usize {
    droplet.surface_area(eventually_adjacent)
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

    pub fn surface_area<T: Fn(&Cubito,&Cubito)->bool>(&self, test: T) -> usize {
        let mut result = 0;
        for cube in self.cubes.iter() {
            let in_contact = self.cubes.iter().filter(|c| test(cube, c)).count();
            assert!(in_contact <= 6);
            result += 6 - in_contact;
        }
        result
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
