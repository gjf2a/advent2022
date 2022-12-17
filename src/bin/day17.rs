use advent_code_lib::{all_lines, simpler_main, Position};
use bare_metal_modulo::*;
use enum_iterator::{all, Sequence};
use std::{
    cmp::{max, min},
    fmt::Display,
};

const WELL_WIDTH: usize = 7;

const PART_1_ITERATIONS: isize = 2022;
const PART_2_ITERATIONS: isize = 1000000000000;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        println!("Iterations/Height at repeat: {:?}", Tetris::find_repeat_iterations_height(filename)?);
        //println!("Part 2: {}", part2(filename)?);
        Ok(())
    })
}

pub fn part1(filename: &str) -> anyhow::Result<isize> {
    Tetris::limit_solver(filename, PART_1_ITERATIONS)
}

//pub fn part2(filename: &str) -> anyhow::Result<isize> {
//    repeat_solver(filename, PART_2_ITERATIONS)
//}

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
    pub fn limit_solver(filename: &str, iterations: isize) -> anyhow::Result<isize> {
        let mut tetris = Self::from_file(filename)?;
        for _ in 0..iterations {
            tetris.drop_next();
        }
        Ok(tetris.height())
    }

    pub fn find_repeat_iterations_height(filename: &str) -> anyhow::Result<(isize,isize)> {
        let mut tetris = Self::from_file(filename)?;
        tetris.drop_next();
        let mut i = 1;
        let start_row = tetris.well.top_row();
        let move_after_start = tetris.moves.get();
        
        while i < 2 || !(tetris.moves.get() == move_after_start && tetris.well.top_row() == start_row && tetris.next_piece() == Tetromino::Plus) {
            tetris.drop_next();
            i += 1;
        }
        tetris.drop_next();
        println!("{}", tetris.well);
        Ok((i, tetris.height()))
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

/* 
pub fn repeat_solver(filename: &str, iterations: isize) -> anyhow::Result<isize> {
    let (repeat_iterations, unit_height) = iterations_height_at_repeat(filename)?;
    let repetitions = iterations / repeat_iterations;
    let extra = iterations % repeat_iterations;
    Ok(repetitions * unit_height + limit_solver(filename, extra)?)
}

pub fn iterations_height_at_repeat(filename: &str) -> anyhow::Result<(isize,isize)> {
    let move_line = read_moves(filename).unwrap();
    let mut moves = moves_from(move_line.as_str());
    let mut w = Well::default();
    let mut tetrominoes = all::<Tetromino>().cycle();
    let mut i = 0;
    while !w.repeats_initial_state() {
        w.drop_into(tetrominoes.next().unwrap(), &mut moves);
        i += 1;
    }
    Ok((i - 1, w.height() - 1))
}
*/
pub fn read_moves(filename: &str) -> anyhow::Result<String> {
    Ok(all_lines(filename)?.next().unwrap())
}

pub fn moves_from(s: &str) -> impl Iterator<Item = Move> + '_ {
    s.chars().map(|c| c.into())
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
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

    pub fn top_row(&self) -> [WellCell; WELL_WIDTH] {
        self.cells.last().cloned().unwrap_or([WellCell::Air; WELL_WIDTH])
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
