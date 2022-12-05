use advent_code_lib::{simpler_main, all_lines};
use std::collections::BTreeSet;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| { 
        let mut total1 = 0;
        let mut total2 = 0;
        let mut trio = vec![];
        for line in all_lines(filename)? {
            total1 += score(char_intersection_1(line.as_str()));
            trio.push(line);
            if trio.len() == 3 {
                total2 += score(char_intersection_2(&trio));
                trio = vec![];
            }
        }
        println!("Part 1: {total1}");
        println!("Part 2: {total2}");
        Ok(())
    })
}

fn score(c: char) -> i64 {
    match c {
        'a'..='z' => c as i64 - 'a' as i64 + 1,
        'A'..='Z' => c as i64 - 'A' as i64 + 27,
        _ => panic!("oops!")
    }
}

fn char_sets_1(line: &str) -> (BTreeSet<char>, BTreeSet<char>) {
    let mut chars = line.chars();
    let left = chars.by_ref().take(line.len() / 2).collect();
    let right = chars.collect();
    (left, right)
}

fn char_intersection_1(line: &str) -> char {
    let (left, right) = char_sets_1(line);
    left.intersection(&right).next().copied().unwrap()
}

fn char_intersection_2(lines: &Vec<String>) -> char {
    assert_eq!(lines.len(), 3);
    let set1 = lines[0].chars().collect::<BTreeSet<_>>();
    let set2 = lines[1].chars().collect::<BTreeSet<_>>();
    let set3 = lines[2].chars().collect::<BTreeSet<_>>();
    let set12 = set1.intersection(&set2).copied().collect::<BTreeSet<_>>();
    set12.intersection(&set3).next().copied().unwrap()
}