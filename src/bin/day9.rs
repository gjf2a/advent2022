use std::collections::BTreeSet;

use advent_code_lib::{all_lines, simpler_main, Dir, Position};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", tail_visit_count::<2>(filename)?);
        println!("Part 2: {}", tail_visit_count::<10>(filename)?);
        Ok(())
    })
}

fn tail_visit_count<const N: usize>(filename: &str) -> anyhow::Result<usize> {
    let mut rope = [Position::new(); N];
    let mut tails = BTreeSet::new();
    for line in all_lines(filename)? {
        let (dir, reps) = parse_line(line.as_str())?;
        for _ in 0..reps {
            rope[0].update(dir);
            for i in 1..N {
                move_if_needed(rope[i - 1], &mut rope[i]);
            }
            tails.insert(rope[N - 1]);
        }
    }
    Ok(tails.len())
}

fn parse_line(line: &str) -> anyhow::Result<(Dir, usize)> {
    let mut parts = line.split_whitespace();
    let dir = match parts.next().unwrap() {
        "R" => Dir::E,
        "U" => Dir::N,
        "L" => Dir::W,
        "D" => Dir::S,
        _ => panic!("I don't recognize this."),
    };
    let reps = parts.next().unwrap().parse::<usize>()?;
    Ok((dir, reps))
}

fn move_if_needed(target: Position, p: &mut Position) {
    if target.neighbors().all(|n| n != *p) {
        if p.row < target.row {
            p.row += 1;
        } else if p.row > target.row {
            p.row -= 1;
        }
        if p.col < target.col {
            p.col += 1;
        } else if p.col > target.col {
            p.col -= 1;
        }
    }
}
