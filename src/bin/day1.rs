use std::io;
use advent_code_lib::{advent_main, all_lines};

fn main() -> io::Result<()> {
    advent_main(&["part1|part2"], &[], |args| {
        let mut elves = vec![];
        let mut elf = 0;
        
        for line in all_lines(args[1].as_str())? {
            let line = line.trim();
            if line.len() == 0 {
                elves.push(elf);
                elf = 0;
            } else {
                elf += line.parse::<i64>().unwrap();
            }
        }
        elves.push(elf);

        let result = if args[2].as_str() == "part1" {part1(&elves)} else {part2(&elves)};

        println!("{result}");
        Ok(())
    })
}

fn part1(elves: &Vec<i64>) -> i64 {
    let mut highest = 0;
    for i in 1..elves.len() {
        if elves[i] > highest {
            highest = elves[i];
        }
    }
    highest
}

fn part2(elves: &Vec<i64>) -> i64 {
    let mut elves = elves.clone();
    elves.sort_by_key(|k| -k);
    elves[..3].iter().sum()
}