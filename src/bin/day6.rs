use std::collections::{BTreeSet, VecDeque};

use advent_code_lib::{all_lines, simpler_main};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let line = all_lines(filename)?.next().unwrap();
        println!("Part 1: {}", scanner(line.as_str(), 4));
        println!("Part 2: {}", scanner(line.as_str(), 14));
        Ok(())
    })
}

fn scanner(line: &str, target_len: usize) -> usize {
    let mut proc_count = 0;
    let mut chars = line.chars();
    let mut marker = VecDeque::new();
    loop {
        marker.push_back(chars.next().unwrap());
        proc_count += 1;
        if marker.len() > target_len {
            marker.pop_front();
        }
        let current = marker.iter().collect::<BTreeSet<_>>();
        if current.len() == target_len {
            return proc_count;
        }
    }
}
