use std::collections::BTreeSet;

use advent_code_lib::{simpler_main, Position, all_lines, Dir};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1: {}", part1(filename)?);
        Ok(())
    })
}

fn part1(filename: &str) -> anyhow::Result<usize> {
    let mut head = Position::new();
    let mut tail = Position::new();
    let mut tails = BTreeSet::new();
    tails.insert(tail);
    for line in all_lines(filename)? {
        let (dir, reps) = parse_line(line.as_str())?;
        for _ in 0..reps {
            let last_head = head;
            head.update(dir);
            if tail != head && head.neighbors().all(|n| n != tail) {
                tail = last_head;
            }
            tails.insert(tail);
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
        _ => panic!("I don't recognize this.")
    };
    let reps = parts.next().unwrap().parse::<usize>()?;
    Ok((dir, reps))
}