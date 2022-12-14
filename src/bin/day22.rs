use std::{
    cmp::{max, min},
    collections::BTreeMap,
    fmt::Display,
    ops::{Index, IndexMut, RangeInclusive},
    str::FromStr,
};

use advent_code_lib::{all_lines, simpler_main, ManhattanDir, Point};
use anyhow::bail;
use enum_iterator::all;

type Pt = Point<isize, 2>;
const CUBE_FACE_NEIGHBORS: usize = 4;
const NUM_CUBE_FACES: usize = 6;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        println!("Part 2: {}", part2(filename)?);
        Ok(())
    })
}

fn find_password<W: PositionWarper>(filename: &str) -> anyhow::Result<isize> {
    let (map, path) = map_path_from::<W>(filename)?;
    let mut mover = map.start();
    println!("Starting at: {mover:?}");
    for path_move in path.path.iter() {
        map.make_move(path_move, &mut mover);
    }
    println!("Ending at: {mover:?}");
    Ok(mover.password())
}

pub fn part1(filename: &str) -> anyhow::Result<isize> {
    find_password::<MapWrapper>(filename)
}

pub fn part2(filename: &str) -> anyhow::Result<isize> {
    find_password::<CubeWrapper>(filename)
}

pub fn map_path_from<W: PositionWarper>(filename: &str) -> anyhow::Result<(Map<W>, Path)> {
    let mut lines = all_lines(filename)?;
    let mut map_lines = vec![];
    loop {
        let line = lines.next().unwrap();
        if line.len() == 0 {
            break;
        }
        map_lines.push(line);
    }
    let instruction_line = lines.next().unwrap();
    assert!(lines.next().is_none());
    Ok((Map::from_lines(map_lines)?, instruction_line.parse()?))
}

pub trait PositionWarper {
    fn new(map: &BTreeMap<Pt, MapCell>, num_rows: isize, num_cols: isize) -> Self;
    fn row2cols(&self) -> &Vec<RangeInclusive<isize>>;
    fn col2rows(&self) -> &Vec<RangeInclusive<isize>>;
    fn update(&self, mover: PathPosition) -> PathPosition;

    fn display_helper(
        &self,
        map: &BTreeMap<Pt, MapCell>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for (row, col_range) in self.row2cols().iter().enumerate() {
            for _ in 0..*col_range.start() {
                write!(f, " ")?;
            }
            for col in col_range.clone() {
                let p = Pt::new([col, row as isize]);
                write!(f, "{}", map.get(&p).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }

    fn starting_column(&self) -> isize {
        *self.row2cols()[0].start()
    }
}

pub struct MapWrapper {
    row2cols: Vec<RangeInclusive<isize>>,
    col2rows: Vec<RangeInclusive<isize>>,
}

impl PositionWarper for MapWrapper {
    fn new(map: &BTreeMap<Pt, MapCell>, num_rows: isize, num_cols: isize) -> Self {
        let row2cols =
            extract_ranges_from(&map, num_rows, num_cols, 0, |outer, inner| [inner, outer]);
        let col2rows =
            extract_ranges_from(&map, num_cols, num_rows, 1, |outer, inner| [outer, inner]);
        Self { row2cols, col2rows }
    }

    fn update(&self, mover: PathPosition) -> PathPosition {
        PathPosition {
            orientation: mover.orientation,
            position: Pt::new(match mover.orientation {
                ManhattanDir::N => [
                    mover.position[0],
                    *self.col2rows[mover.position[0] as usize].end(),
                ],
                ManhattanDir::E => [
                    *self.row2cols[mover.position[1] as usize].start(),
                    mover.position[1],
                ],
                ManhattanDir::S => [
                    mover.position[0],
                    *self.col2rows[mover.position[0] as usize].start(),
                ],
                ManhattanDir::W => [
                    *self.row2cols[mover.position[1] as usize].end(),
                    mover.position[1],
                ],
            }),
        }
    }

    fn row2cols(&self) -> &Vec<RangeInclusive<isize>> {
        &self.row2cols
    }

    fn col2rows(&self) -> &Vec<RangeInclusive<isize>> {
        &self.col2rows
    }
}

pub struct CubeWrapper {
    row2cols: Vec<RangeInclusive<isize>>,
    col2rows: Vec<RangeInclusive<isize>>,
    cube: Vec<CubeFace>,
}

impl CubeWrapper {
    fn cube_index_for(&self, mover: PathPosition) -> Option<usize> {
        for (i, face) in self.cube.iter().enumerate() {
            if face.contains(mover.position) {
                return Some(i);
            }
        }
        None
    }
}

impl PositionWarper for CubeWrapper {
    fn new(map: &BTreeMap<Pt, MapCell>, num_rows: isize, num_cols: isize) -> Self {
        let row2cols =
            extract_ranges_from(&map, num_rows, num_cols, 0, |outer, inner| [inner, outer]);
        let col2rows =
            extract_ranges_from(&map, num_cols, num_rows, 1, |outer, inner| [outer, inner]);
        let mut cube = cube_from(&row2cols, num_rows, num_cols);
        assert_eq!(cube.len(), NUM_CUBE_FACES);
        resolve_easy_neighbors(&mut cube);
        resolve_cube(&mut cube);
        println!("resolved...");
        print_cube(&cube);
        Self {
            row2cols,
            col2rows,
            cube,
        }
    }

    fn row2cols(&self) -> &Vec<RangeInclusive<isize>> {
        &self.row2cols
    }

    fn col2rows(&self) -> &Vec<RangeInclusive<isize>> {
        &self.col2rows
    }

    fn update(&self, mover: PathPosition) -> PathPosition {
        let i_start_face = self.cube_index_for(mover).unwrap();
        let start_face = self.cube[i_start_face].clone();
        let i_end_face = start_face[mover.orientation].unwrap();
        let end_face = self.cube[i_end_face].clone();
        let end2start = end_face.dir_to(i_start_face).unwrap();
        let new_orientation = end2start.inverse();
        let left_departure = start_face.left_departure_offset(mover);
        let new_pt = end_face.receiving_position(end2start, left_departure);
        PathPosition {
            position: new_pt,
            orientation: new_orientation,
        }
    }
}

fn cube_from(
    row2cols: &Vec<RangeInclusive<isize>>,
    num_rows: isize,
    num_cols: isize,
) -> Vec<CubeFace> {
    let cube_size = (min(num_rows, num_cols) / 3) as usize;
    let face_offset = cube_size as isize - 1;
    assert_eq!(cube_size as isize * 3, min(num_rows, num_cols));
    let mut cube = vec![];
    for (row, cols) in row2cols.iter().enumerate().step_by(cube_size) {
        for start in cols.clone().step_by(cube_size) {
            let row = row as isize;
            cube.push(CubeFace::new(
                start..=start + face_offset,
                row..=row + face_offset,
            ));
        }
    }
    cube
}

fn resolve_cube(cube: &mut Vec<CubeFace>) {
    for _ in 0..10 {
        for face in 0..cube.len() {
            for dir in all::<ManhattanDir>() {
                if let Some(n1) = cube[face][dir] {
                    fix_corner(cube, face, dir, n1, |d| d.clockwise());
                    fix_corner(cube, face, dir, n1, |d| d.counterclockwise());
                }
            }
        }
    }
}

fn fix_corner<D: Fn(ManhattanDir) -> ManhattanDir>(
    cube: &mut Vec<CubeFace>,
    face: usize,
    dir: ManhattanDir,
    n1: usize,
    next_dir: D,
) {
    if let Some(n2) = cube[face][next_dir(dir)] {
        let back = next_dir(cube[n2].dir_to(face).unwrap());
        if cube[n1][next_dir(dir)].is_none() && cube[n2][back].is_none() {
            cube[n1][next_dir(dir)] = Some(n2);
            cube[n2][back] = Some(n1);
        }
    }
}

fn resolve_easy_neighbors(cube: &mut Vec<CubeFace>) {
    for i in 0..cube.len() {
        for j in i + 1..cube.len() {
            if let Some(dir) = cube[i].touches(&cube[j]) {
                cube[i][dir] = Some(j);
                cube[j][dir.inverse()] = Some(i);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CubeFace {
    xs: RangeInclusive<isize>,
    ys: RangeInclusive<isize>,
    neighbors: [Option<usize>; CUBE_FACE_NEIGHBORS],
}

impl CubeFace {
    pub fn new(xs: RangeInclusive<isize>, ys: RangeInclusive<isize>) -> Self {
        Self {
            xs,
            ys,
            neighbors: [None; CUBE_FACE_NEIGHBORS],
        }
    }

    pub fn dir_to(&self, neighbor: usize) -> Option<ManhattanDir> {
        all::<ManhattanDir>().find(|d| self.neighbors[*d as usize] == Some(neighbor))
    }

    pub fn has_all_neighbors(&self) -> bool {
        self.neighbors.iter().all(|n| n.is_some())
    }

    pub fn matched_neighbors(&self) -> impl Iterator<Item = ManhattanDir> + '_ {
        all::<ManhattanDir>().filter(|d| self.neighbors[*d as usize].is_some())
    }

    pub fn contains(&self, p: Pt) -> bool {
        self.xs.contains(&p[0]) && self.ys.contains(&p[1])
    }

    pub fn touches(&self, other: &CubeFace) -> Option<ManhattanDir> {
        for dir in all::<ManhattanDir>() {
            let test_point = match dir {
                ManhattanDir::N => Pt::new([*self.xs.start(), *self.ys.start() - 1]),
                ManhattanDir::E => Pt::new([*self.xs.end() + 1, *self.ys.start()]),
                ManhattanDir::S => Pt::new([*self.xs.start(), *self.ys.end() + 1]),
                ManhattanDir::W => Pt::new([*self.xs.start() + 1, *self.ys.start()]),
            };
            if other.contains(test_point) {
                return Some(dir);
            }
        }
        None
    }

    pub fn left_departure_offset(&self, departer: PathPosition) -> isize {
        match departer.orientation {
            ManhattanDir::N => departer.position[0] - self.xs.start(),
            ManhattanDir::E => departer.position[1] - self.ys.start(),
            ManhattanDir::S => self.xs.end() - departer.position[0],
            ManhattanDir::W => self.ys.end() - departer.position[1],
        }
    }

    pub fn receiving_position(
        &self,
        receiving_direction: ManhattanDir,
        left_departure: isize,
    ) -> Pt {
        Pt::new(match receiving_direction {
            ManhattanDir::N => [self.xs.end() - left_departure, *self.ys.start()],
            ManhattanDir::E => [*self.xs.end(), self.ys.end() - left_departure],
            ManhattanDir::S => [self.xs.start() + left_departure, *self.ys.end()],
            ManhattanDir::W => [*self.xs.start(), self.ys.start() + left_departure],
        })
    }
}

impl Index<ManhattanDir> for CubeFace {
    type Output = Option<usize>;

    fn index(&self, index: ManhattanDir) -> &Self::Output {
        &self.neighbors[index as usize]
    }
}

impl IndexMut<ManhattanDir> for CubeFace {
    fn index_mut(&mut self, index: ManhattanDir) -> &mut Self::Output {
        &mut self.neighbors[index as usize]
    }
}

impl Display for CubeFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xs: {:?} ys: {:?}", self.xs, self.ys)?;
        for dir in all::<ManhattanDir>() {
            if let Some(n) = self[dir] {
                write!(f, " {:?}:{}", dir, n + 1)?;
            }
        }
        Ok(())
    }
}

fn print_cube(cube: &Vec<CubeFace>) {
    for i in 0..cube.len() {
        println!("Face {}: {}", i + 1, cube[i]);
    }
}

#[derive(Debug, Clone)]
pub struct Map<W> {
    map: BTreeMap<Pt, MapCell>,
    warper: W,
}

impl<W: PositionWarper> Map<W> {
    pub fn from_lines(lines: Vec<String>) -> anyhow::Result<Self> {
        let mut map = BTreeMap::new();
        let mut longest_line_len = 0;
        for (row, line) in lines.iter().enumerate() {
            longest_line_len = max(longest_line_len, line.len());
            for (col, c) in line.chars().enumerate() {
                if ['#', '.'].contains(&c) {
                    let p = Pt::new([col as isize, row as isize]);
                    let cell = if c == '#' {
                        MapCell::Wall
                    } else {
                        MapCell::Space
                    };
                    map.insert(p, cell);
                } else if c != ' ' {
                    bail!("{c} is illegal")
                }
            }
        }
        let warper = W::new(&map, lines.len() as isize, longest_line_len as isize);
        Ok(Map { map, warper })
    }

    pub fn start(&self) -> PathPosition {
        PathPosition {
            position: Pt::new([self.warper.starting_column(), 0]),
            orientation: ManhattanDir::E,
        }
    }

    pub fn make_move(&self, path_move: &PathMove, mover: &mut PathPosition) {
        match path_move {
            PathMove::Left => {
                mover.orientation = mover.orientation.counterclockwise();
            }
            PathMove::Right => {
                mover.orientation = mover.orientation.clockwise();
            }
            PathMove::Forward(distance) => {
                let mut countdown = *distance;
                while countdown > 0 {
                    let mut next = *mover;
                    next.position.manhattan_move(mover.orientation);
                    if !self.map.contains_key(&next.position) {
                        next = self.warper.update(*mover);
                    }
                    let cell = self.map.get(&next.position).unwrap();
                    if *cell == MapCell::Space {
                        *mover = next;
                    }
                    countdown -= 1;
                }
            }
        }
    }
}

fn extract_ranges_from<F: Fn(isize, isize) -> [isize; 2]>(
    map: &BTreeMap<Pt, MapCell>,
    outer: isize,
    inner: isize,
    index: usize,
    indexer: F,
) -> Vec<RangeInclusive<isize>> {
    let mut result = vec![];
    for i in 0..outer {
        let mut min_v = inner;
        let mut max_v = 0;
        for j in 0..inner {
            let p = Pt::new(indexer(i, j));
            if map.contains_key(&p) {
                min_v = min(min_v, p[index]);
                max_v = max(max_v, p[index]);
            }
        }
        result.push(min_v..=max_v);
    }
    result
}

impl<W: PositionWarper> Display for Map<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.warper.display_helper(&self.map, f)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MapCell {
    Wall,
    Space,
}

impl Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Wall => '#',
                Self::Space => '.',
            }
        )
    }
}

pub struct Path {
    path: Vec<PathMove>,
}

impl FromStr for Path {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s
            .split(['R', 'L'])
            .map(|ns| ns.parse::<PathMove>().unwrap());
        let mut moves = s
            .split(|c: char| c.is_digit(10))
            .filter(|ms| ms.len() > 0)
            .map(|ms| ms.parse::<PathMove>().unwrap());
        let mut path = vec![];
        while let Some(num) = nums.next() {
            path.push(num);
            if let Some(m) = moves.next() {
                path.push(m);
            }
        }
        Ok(Path { path })
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in self.path.iter() {
            write!(f, "{m}")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PathMove {
    Forward(isize),
    Left,
    Right,
}

impl FromStr for PathMove {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => match s.parse::<isize>() {
                Ok(n) => Ok(Self::Forward(n)),
                Err(e) => bail!(e),
            },
        }
    }
}

impl Display for PathMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Left => "L".to_owned(),
            Self::Right => "R".to_owned(),
            Self::Forward(n) => format!("{n}"),
        };
        write!(f, "{s}",)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PathPosition {
    position: Pt,
    orientation: ManhattanDir,
}

impl Display for PathPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.orientation, self.position)
    }
}

impl PathPosition {
    pub fn password(&self) -> isize {
        let facing = match self.orientation {
            ManhattanDir::N => 3,
            ManhattanDir::E => 0,
            ManhattanDir::S => 1,
            ManhattanDir::W => 2,
        };
        1000 * (1 + self.position[1]) + 4 * (1 + self.position[0]) + facing
    }
}

#[cfg(test)]
mod tests {
    use crate::{map_path_from, MapWrapper};

    #[test]
    fn test_parse() {
        let (map, path) = map_path_from::<MapWrapper>("ex/day22.txt").unwrap();
        assert_eq!(format!("{path}"), "10R5L5R10L4R5L5");
        assert_eq!(
            format!("{map}"),
            "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.
"
        );
    }
}
