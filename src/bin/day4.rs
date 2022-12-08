use advent_code_lib::{all_lines, simpler_main};
use std::ops::RangeInclusive;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let mut total1 = 0;
        let mut total2 = 0;
        for line in all_lines(filename)? {
            let (elf1, elf2) = parse_ranges(line.as_str());
            if fully_contains(&elf1, &elf2) || fully_contains(&elf2, &elf1) {
                total1 += 1;
            }
            if overlap(&elf1, &elf2) {
                total2 += 1;
            }
        }
        println!("Part 1: {total1}");
        println!("Part 2: {total2}");
        Ok(())
    })
}

fn parse_ranges(line: &str) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
    let mut parts = line.split(",");
    (
        parse_range(parts.next().unwrap()),
        parse_range(parts.next().unwrap()),
    )
}

fn parse_range(range: &str) -> RangeInclusive<i64> {
    let mut parts = range.split("-");
    let start = parts.next().unwrap().parse::<i64>().unwrap();
    let end = parts.next().unwrap().parse::<i64>().unwrap();
    start..=end
}

fn fully_contains(container: &RangeInclusive<i64>, containee: &RangeInclusive<i64>) -> bool {
    container.contains(containee.start()) && container.contains(containee.end())
}

fn overlap(elf1: &RangeInclusive<i64>, elf2: &RangeInclusive<i64>) -> bool {
    elf1.contains(elf2.start())
        || elf1.contains(elf2.end())
        || elf2.contains(elf1.start())
        || elf2.contains(elf1.end())
}
