use advent_code_lib::{all_lines, simpler_main, Position};
use enum_iterator::Sequence;
use std::{
    cmp::{max, min},
    fmt::Display,
};

const WELL_WIDTH: usize = 7;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| Ok(()))
}

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
        write!(
            f,
            "{c}",
            
        )
    }
}

#[derive(Default)]
pub struct Well<const W: usize> {
    cells: Vec<[WellCell; W]>,
}

impl<const W: usize> Display for Well<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.cells.len()).rev() {
            write!(f, "|")?;
            for c in self.cells[i].iter() {
                write!(f, "{c}")?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "+")?;
        for _ in 0..W {
            write!(f, "-")?;
        }
        writeln!(f, "+")
    }
}

impl<const W: usize> Well<W> {
    pub fn height(&self) -> isize {
        self.cells.len() as isize
    }

    pub fn contacts(&self, t: Tetromino, p: Position) -> bool {
        p.row == 0 || t.right_column(p) == (W as isize - 1) ||
        t.positions(p)
            .filter(|tp| tp.row < self.height())
            .any(|tp| {
                self.cells[tp.row as usize]
                        .iter()
                        .enumerate()
                        .any(|(i, c)| *c == WellCell::Rock && i as isize == tp.col)
            })
    }

    pub fn drop_into<I: Iterator<Item = Move>>(&mut self, t: Tetromino, moves: &mut I) {
        let mut tp = Position {
            col: 2,
            row: self.height() + 3,
        };
        while !self.contacts(t, tp) {
            println!("tp: {tp}");
            tp = moves.next().unwrap().push::<W>(tp);
            tp.row -= 1;
        }
        for rock in t.positions(tp) {
            while rock.row >= self.height() {
                self.cells.push([WellCell::Air; W]);
            }
            self.cells[rock.row as usize][rock.col as usize] = WellCell::Rock;
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

    pub fn right_column(&self, bottom_left: Position) -> isize {
        self.positions(bottom_left).map(|p| p.col).max().unwrap()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Move {
    Left,
    Right,
}

impl Move {
    pub fn push<const W: usize>(&self, p: Position) -> Position {
        match self {
            Self::Left => Position {
                row: p.row,
                col: max(0, p.col - 1),
            },
            Self::Right => Position {
                row: p.row,
                col: min(W as isize, p.col + 1),
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
        write!(
            f,
            "{}",
            match self {
                Self::Left => '<',
                Self::Right => '>',
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use enum_iterator::all;

    use crate::{moves_from, read_moves, Tetromino, Well, WELL_WIDTH};

    #[test]
    fn test_empty() {
        let w = Well::<WELL_WIDTH>::default();
        assert_eq!("+-------+\n", format!("{w}"));
    }

    #[test]
    fn test_drop() {
        for t in all::<Tetromino>() {
            let move_line = read_moves("ex/day17.txt").unwrap();
            let mut moves = moves_from(move_line.as_str());
            let mut w = Well::<WELL_WIDTH>::default();
            w.drop_into(t, &mut moves);
            println!("{w}");
        }
    }
}
