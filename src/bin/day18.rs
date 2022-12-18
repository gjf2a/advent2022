use advent_code_lib::{all_lines, simpler_main, Point};

type Cubito = Point<i64, 3>;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let droplet = Droplet::from_file(filename)?;
        println!("Part 1: {}", droplet.surface_area());
        Ok(())
    })
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

    pub fn surface_area(&self) -> usize {
        let mut result = 0;
        for cube in self.cubes.iter() {
            let in_contact = self.cubes.iter().filter(|c| cube.adjacent(c)).count();
            assert!(in_contact <= 6);
            result += 6 - in_contact;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Droplet;

    #[test]
    fn test_1() {
        let droplet = Droplet::from_file("ex/day18.txt").unwrap();
        assert_eq!(droplet.surface_area(), 64);
    }
}
