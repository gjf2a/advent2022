use std::collections::BTreeSet;

use advent_code_lib::{simpler_main, Position, all_lines, Dir};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        println!("Part 1a: {}", part1(filename)?);
        println!("Part 1b: {}", tail_visit_count::<2>(filename)?);
        println!("Part 2:  {}", tail_visit_count::<10>(filename)?);
        Ok(())
    })
}

fn print_array<const N: usize>(array: &[Position; N]) {
    for p in array.iter() {
        print!(" ({},{})", p.row, p.col);
    }
    println!();
}

fn part1(filename: &str) -> anyhow::Result<usize> {
    let mut head = Position::new();
    let mut tail = Position::new();
    let mut tails = BTreeSet::new();
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

fn tail_visit_count<const N: usize>(filename: &str) -> anyhow::Result<usize> {
    let mut rope = [Position::new(); N];
    let mut tails = BTreeSet::new();
    for line in all_lines(filename)? {
        let (dir, reps) = parse_line(line.as_str())?;
        for r in 0..reps {
            let last_rope = rope;
            rope[0].update(dir);
            for i in 1..N {
                if rope[i] != rope[i - 1] && rope[i - 1].neighbors().all(|n| n != rope[i]) {
                    rope[i] = last_rope[i - 1];
                }
            }
            tails.insert(rope[N - 1]);
            print!("{dir:?} {}", r + 1);
            print_array(&rope);
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