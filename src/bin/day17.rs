use advent_code_lib::{all_lines, simpler_main, Position};
use bare_metal_modulo::*;
use enum_iterator::{all, Sequence};
use std::{
    cmp::{max, min},
    fmt::{Debug, Display}, collections::HashMap,
};

const WELL_WIDTH: usize = 7;

const PART_1_ITERATIONS: isize = 2022;
const PART_2_ITERATIONS: isize = 1000000000000;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1:  {}", part1(filename)?);
        let repeat_data = Tetris::find_repeat_iterations_height(filename)?;
        let t = Tetris::build_to_limit(filename, repeat_data.start_drops + repeat_data.repetition_drops * 2)?;
        print!("{}", t.well);
        println!("{repeat_data:?}");
        println!("Reprise: {}", repeat_data.calculate_height_at(filename, PART_1_ITERATIONS)?);
        println!("Part 2:  {}", repeat_data.calculate_height_at(filename, PART_2_ITERATIONS)?);
        Ok(())
    })
}

pub fn part1(filename: &str) -> anyhow::Result<isize> {
    Tetris::limit_solver(filename, PART_1_ITERATIONS)
}

pub struct Tracker<T> {
    items: Vec<T>,
    track: ModNum<usize>,
}

impl <T:Copy> Tracker<T> {
    pub fn new<I: Iterator<Item=T>>(items: I) -> Self {
        let items: Vec<T> = items.collect();
        let track = ModNum::new(0, items.len());
        Self {items, track}
    }

    pub fn get(&self) -> T {
        self.items[self.track.a()]
    }

    pub fn i(&self) -> usize {
        self.track.a()
    }

    pub fn advance(&mut self) {
        self.track += 1;
    }
}

pub struct Tetris {
    well: Well,
    moves: Tracker<Move>,
    pieces: Tracker<Tetromino>,
}

impl Tetris {
    pub fn build_to_limit(filename: &str, iterations: isize) -> anyhow::Result<Self> {
        let mut tetris = Self::from_file(filename)?;
        for _ in 0..iterations {
            tetris.drop_next();
        }
        Ok(tetris)
    } 

    pub fn limit_solver(filename: &str, iterations: isize) -> anyhow::Result<isize> {
        Self::build_to_limit(filename, iterations).map(|t| t.height())
    }

    pub fn find_repeat_iterations_height(filename: &str) -> anyhow::Result<RepeatOutcome> {
        let mut previous_rows = HashMap::new();
        let mut tetris = Self::from_file(filename)?;
        let mut num_drops = 0;
        loop {
            tetris.drop_next();
            num_drops += 1;
            let top_row = tetris.well.top_row();
            match previous_rows.get_mut(&top_row) {
                None => {previous_rows.insert(top_row, vec![Checkpoint {num_drops, height: tetris.height()}]);}
                Some(repeat) => {
                    for checkpoint in repeat.iter().rev() {
                        if checkpoint.height < tetris.height() {
                            if tetris.well.repetition_of(checkpoint.height - 1, tetris.height() - 1) {
                                return Ok(RepeatOutcome { start_drops: checkpoint.num_drops, repetition_drops: num_drops - checkpoint.num_drops, repetition_length: tetris.height() - checkpoint.height });
                            }
                        }
                    }
                    let mut last = repeat.last_mut().unwrap();
                    if last.height == tetris.height() {
                        last.num_drops = num_drops;
                    } else {
                        repeat.push(Checkpoint {num_drops, height: tetris.height()});
                    }
                }
            }
        }
    }

    pub fn height(&self) -> isize {
        self.well.height()
    }

    pub fn next_piece(&self) -> Tetromino {
        self.pieces.get()
    }

    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let move_line = read_moves(filename)?;
        let moves = Tracker::new(move_line.chars().map(|c| c.into()));
        let pieces = Tracker::new(all::<Tetromino>());
        Ok(Self {
            well: Well::default(),
            moves,
            pieces,
        })
    }

    pub fn drop_next(&mut self) {
        self.well.drop_into(self.pieces.get(), &mut self.moves);
        self.pieces.advance();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepeatOutcome {
    pub start_drops: isize,
    pub repetition_drops: isize,
    pub repetition_length: isize,
}

impl RepeatOutcome {
    pub fn calculate_height_at(&self, filename: &str, iterations: isize) -> anyhow::Result<isize> {
        let total = iterations - self.start_drops;
        let num_repetitions = total / self.repetition_drops;
        let extra_drops = total % self.repetition_drops;
        println!("extra: {extra_drops}");
        let drops_to_simulate = self.start_drops + self.repetition_drops + extra_drops;
        let base_height = Tetris::limit_solver(filename, drops_to_simulate)?;
        let extra_height = self.repetition_length * (num_repetitions - 1);
        Ok(base_height + extra_height)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    num_drops: isize,
    height: isize,
}

pub fn read_moves(filename: &str) -> anyhow::Result<String> {
    Ok(all_lines(filename)?.next().unwrap())
}

pub fn moves_from(s: &str) -> impl Iterator<Item = Move> + '_ {
    s.chars().map(|c| c.into())
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug, Hash)]
pub enum WellCell {
    Rock,
    #[default]
    Air,
}

impl Display for WellCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            WellCell::Rock => "#",
            WellCell::Air => ".",
        };
        write!(f, "{c}")
    }
}

#[derive(Default)]
pub struct Well {
    cells: Vec<[WellCell; WELL_WIDTH]>,
}

impl Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.cells.len()).rev() {
            write!(f, "|")?;
            for c in self.cells[i].iter() {
                write!(f, "{c}")?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "+")?;
        for _ in 0..WELL_WIDTH {
            write!(f, "-")?;
        }
        writeln!(f, "+")
    }
}

impl Well {
    pub fn at(&self, p: Position) -> WellCell {
        if p.row >= self.height() {
            WellCell::Air
        } else {
            self.cells[p.row as usize][p.col as usize]
        }
    }

    pub fn repetition_of(&self, end1: isize, end2: isize) -> bool {
        let size = end2 - end1;
        if size <= 0 {
            false
        } else {
            let mut two = end2;
            let mut one = end1;
            while two > end1 {
                if one < 0 || self.cells[one as usize] != self.cells[two as usize] {
                    return false;
                }
                one -= 1;
                two -= 1;
            }
            true
        }
    }

    pub fn top_row(&self) -> [WellCell; WELL_WIDTH] {
        self.cells.last().cloned().unwrap_or([WellCell::Air; WELL_WIDTH])
    }

    pub fn row(&self, height: isize) -> [WellCell; WELL_WIDTH] {
        self.cells[height as usize].clone()
    }

    pub fn height(&self) -> isize {
        self.cells.len() as isize
    }

    pub fn contacts(&self, t: Tetromino, p: Position) -> bool {
        p.row < 0
            || t.positions(p)
                .filter(|tp| tp.row < self.height())
                .any(|tp| {
                    self.cells[tp.row as usize]
                        .iter()
                        .enumerate()
                        .any(|(i, c)| *c == WellCell::Rock && i as isize == tp.col)
                })
    }

    pub fn drop_into(&mut self, t: Tetromino, moves: &mut Tracker<Move>) {
        let mut tp = Position {
            col: 2,
            row: self.height() + 3,
        };
        loop {
            if let Some(new_tp) = self.push(t, tp, moves.get()) {
                tp = new_tp;
            }
            moves.advance();
            if self.contacts(t, tp - Position { row: 1, col: 0 }) {
                break;
            }
            tp.row -= 1;
        }
        for rock in t.positions(tp) {
            while rock.row >= self.height() {
                self.cells.push([WellCell::Air; WELL_WIDTH]);
            }
            self.cells[rock.row as usize][rock.col as usize] = WellCell::Rock;
        }
    }

    fn push(&self, t: Tetromino, tp: Position, m: Move) -> Option<Position> {
        let new_tp = m.push(tp, t.width());
        if t.positions(new_tp).any(|p| self.at(p) == WellCell::Rock) {
            None
        } else {
            Some(new_tp)
        }
    }

    pub fn add_row(&mut self, chars: &str) {
        self.cells.push([WellCell::Air; WELL_WIDTH]);
        for (i, c) in chars.chars().enumerate() {
            if c == '#' {
                self.cells.last_mut().unwrap()[i] = WellCell::Rock;
            }
        }
    }
}

#[derive(Copy, Clone, Sequence, Eq, PartialEq, Debug)]
pub enum Tetromino {
    Minus,
    Plus,
    ReverseL,
    Or,
    Square,
}

impl Tetromino {
    pub fn positions(&self, bottom_left: Position) -> impl Iterator<Item = Position> {
        match self {
            Tetromino::Minus => [(0, 0), (1, 0), (2, 0), (3, 0)].iter(),
            Tetromino::Plus => [(1, 0), (1, 1), (0, 1), (2, 1), (1, 2)].iter(),
            Tetromino::ReverseL => [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)].iter(),
            Tetromino::Or => [(0, 0), (0, 1), (0, 2), (0, 3)].iter(),
            Tetromino::Square => [(0, 0), (0, 1), (1, 0), (1, 1)].iter(),
        }
        .map(move |(x, y)| bottom_left + Position { col: *x, row: *y })
    }

    pub fn width(&self) -> isize {
        self.positions(Position { col: 1, row: 0 })
            .map(|p| p.col)
            .max()
            .unwrap()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Move {
    Left,
    Right,
}

impl Move {
    pub fn push(&self, p: Position, width: isize) -> Position {
        match self {
            Self::Left => Position {
                row: p.row,
                col: max(0, p.col - 1),
            },
            Self::Right => Position {
                row: p.row,
                col: min(WELL_WIDTH as isize - width, p.col + 1),
            },
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!("{c} is not a Move"),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Left => '<',
            Self::Right => '>',
        };
        write!(f, "{c}")
    }
}

#[cfg(test)]
mod tests {
    use enum_iterator::all;

    use crate::{moves_from, read_moves, Tetromino, Well, Tracker};

    #[test]
    fn test_empty() {
        let w = Well::default();
        assert_eq!("+-------+\n", format!("{w}"));
    }

    const EX_1: &str = "|....##.|
|....##.|
|....#..|
|..#.#..|
|..#.#..|
|#####..|
|..###..|
|...#...|
|..####.|
+-------+
";
    #[test]
    fn test_drop() {
        let move_line = read_moves("ex/day17.txt").unwrap();
        let mut moves = Tracker::new(moves_from(move_line.as_str()));
        let mut w = Well::default();
        for t in all::<Tetromino>() {
            w.drop_into(t, &mut moves);
        }
        assert_eq!(format!("{w}"), EX_1);
    }
}
