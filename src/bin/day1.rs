use advent_code_lib::{all_lines, simpler_main};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let mut elves = vec![];
        let mut elf = 0;

        for line in all_lines(filename)? {
            let line = line.trim();
            if line.len() == 0 {
                elves.push(elf);
                elf = 0;
            } else {
                elf += line.parse::<i64>().unwrap();
            }
        }
        elves.push(elf);

        println!("part 1: {}", elves.iter().max().unwrap());
        println!("part 2: {}", part2(&elves));
        Ok(())
    })
}

fn part2(elves: &Vec<i64>) -> i64 {
    let mut elves = elves.clone();
    elves.sort_by_key(|k| -k);
    elves[..3].iter().sum()
}
