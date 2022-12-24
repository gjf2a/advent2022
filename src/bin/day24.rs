use std::{
    cmp::max,
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

use advent_code_lib::{all_lines, simpler_main, ManhattanDir, Point};
use enum_iterator::all;

type Pt = Point<isize, 2>;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        //test(filename, 5)?;
        println!("Part 1: {}", part1(filename)?);
        println!("Part 2: {}", part2(filename)?);
        Ok(())
    })
}

fn test(filename: &str, iterations: usize) -> anyhow::Result<()> {
    let mut map = BlizzardMap::from_file(filename)?;
    println!("{map}");
    for _ in 0..iterations {
        println!();
        map = map.next_step();
        println!("{map}");
    }
    Ok(())
}

fn part1(filename: &str) -> anyhow::Result<usize> {
    let map = BlizzardMap::from_file(filename)?;
    Ok(Reachability::minutes2exit(map))
}

fn part2(filename: &str) -> anyhow::Result<usize> {
    Ok(0)
}

struct Reachability {
    minute2reachable: Vec<BTreeSet<Pt>>,
}

impl Reachability {
    fn new(map: &BlizzardMap) -> Self {
        let mut minute_zero = BTreeSet::new();
        minute_zero.insert(map.entrance());
        Self {
            minute2reachable: vec![minute_zero],
        }
    }

    fn current(&self) -> &BTreeSet<Pt> {
        self.minute2reachable.last().unwrap()
    }

    fn elapsed_minutes(&self) -> usize {
        self.minute2reachable.len() - 1
    }

    fn iterate(&mut self, map: &mut BlizzardMap) {
        *map = map.next_step();
        let mut reachable = BTreeSet::new();
        for prev in self.current().iter() {
            if map.can_enter(*prev) {
                reachable.insert(*prev);
            }
            for neighbor in prev.manhattan_neighbors() {
                if map.can_enter(neighbor) {
                    reachable.insert(neighbor);
                }
            }
        }
        self.minute2reachable.push(reachable);
    }

    fn minutes2exit(mut map: BlizzardMap) -> usize {
        let mut reachability = Self::new(&map);
        while !reachability.current().contains(&map.exit()) {
            reachability.iterate(&mut map);
        }
        reachability.elapsed_minutes()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum BlizzardCell {
    Wall,
    Wind(Wind),
}

impl BlizzardCell {
    fn add_wind(&mut self, wind_dir: ManhattanDir) {
        match self {
            Self::Wall => {
                panic!("Illegal operation")
            }
            Self::Wind(w) => {
                w.add_wind(wind_dir);
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Default)]
struct Wind {
    has_wind: [bool; 4],
}

impl Wind {
    fn dir(dir: ManhattanDir) -> Self {
        let mut result = Self::default();
        result.has_wind[dir as usize] = true;
        result
    }
}

impl Display for Wind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let winds = self.winds();
        let count_str = format!("{}", winds.len());
        let s = match winds.len() {
            0 => ".",
            1 => match winds[0] {
                ManhattanDir::N => "^",
                ManhattanDir::E => ">",
                ManhattanDir::S => "v",
                ManhattanDir::W => "<",
            },
            _ => count_str.as_str(),
        };
        write!(f, "{s}")
    }
}

impl Wind {
    fn is_windy(&self) -> bool {
        self.has_wind.iter().any(|w| *w)
    }

    fn winds(&self) -> Vec<ManhattanDir> {
        all::<ManhattanDir>()
            .filter(|d| self.has_wind[*d as usize])
            .collect()
    }

    fn add_wind(&mut self, wind_dir: ManhattanDir) {
        self.has_wind[wind_dir as usize] = true;
    }
}

impl Display for BlizzardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlizzardCell::Wall => write!(f, "#"),
            BlizzardCell::Wind(w) => write!(f, "{}", *w),
        }
    }
}

impl From<char> for BlizzardCell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Wind(Wind::default()),
            '#' => Self::Wall,
            '^' => Self::Wind(Wind::dir(ManhattanDir::N)),
            '>' => Self::Wind(Wind::dir(ManhattanDir::E)),
            'v' => Self::Wind(Wind::dir(ManhattanDir::S)),
            '<' => Self::Wind(Wind::dir(ManhattanDir::W)),
            _ => panic!("Illegal character"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BlizzardMap {
    wind_map: BTreeMap<Pt, BlizzardCell>,
    width: isize,
    height: isize,
}

impl BlizzardMap {
    fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut wind_map = BTreeMap::new();
        let mut height = 0;
        let mut width = 0;
        for (row, line) in all_lines(filename)?.enumerate() {
            for (col, c) in line.chars().enumerate() {
                width = max(width, col as isize + 1);
                wind_map.insert(Pt::new([col as isize, row as isize]), c.into());
            }
            height += 1;
        }
        Ok(Self {
            wind_map,
            width,
            height,
        })
    }

    fn can_enter(&self, p: Pt) -> bool {
        self.wind_map.get(&p).map_or(false, |c| match c {
            BlizzardCell::Wall => false,
            BlizzardCell::Wind(w) => !w.is_windy(),
        })
    }

    fn entrance(&self) -> Pt {
        Pt::new([1, 0])
    }

    fn exit(&self) -> Pt {
        Pt::new([self.width - 2, self.height - 1])
    }

    fn next_step(&self) -> Self {
        let mut next = Self {
            width: self.width,
            height: self.height,
            wind_map: self
                .wind_map
                .iter()
                .map(|(p, c)| {
                    (
                        *p,
                        match c {
                            BlizzardCell::Wall => *c,
                            BlizzardCell::Wind(_) => BlizzardCell::Wind(Wind::default()),
                        },
                    )
                })
                .collect(),
        };
        for (p, cell) in self.wind_map.iter() {
            if let BlizzardCell::Wind(w) = cell {
                for wind_dir in w.winds() {
                    let next_p = self.wind_next(wind_dir, *p);
                    next.wind_map.get_mut(&next_p).unwrap().add_wind(wind_dir);
                }
            }
        }
        next
    }

    fn wind_next(&self, wind_dir: ManhattanDir, wind_pos: Pt) -> Pt {
        let mut updated = wind_pos.manhattan_moved(wind_dir);
        let max_x = self.width - 2;
        let max_y = self.height - 2;
        if updated[0] == 0 {
            updated[0] = max_x;
        }
        if updated[1] == 0 {
            updated[1] = max_y;
        }
        if updated[0] > max_x {
            updated[0] = 1;
        }
        if updated[1] > max_y {
            updated[1] = 1;
        }
        updated
    }
}

impl Display for BlizzardMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.wind_map.get(&Pt::new([col, row])).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
