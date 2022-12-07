use std::collections::BTreeMap;

use advent_code_lib::{simpler_main, all_lines};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| { 
        let system = FileSystem::from_file(filename)?;
        println!("Part 1: {}", system.part1());
        Ok(())
    })
}

#[derive(Debug)]
pub struct FileSystem {
    directories: BTreeMap<String,(Option<String>,Vec<String>)>,
    files: BTreeMap<String,(String,usize)>
}

impl FileSystem {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut system = FileSystem {directories: BTreeMap::new(), files: BTreeMap::new()};
        system.directories.insert("/".to_owned(), (None, vec![]));
        let mut current_dir = "/".to_owned();
        for line in all_lines(filename)? {
            if line == "$ cd .." {
                let info = system.directories.get(current_dir.as_str()).unwrap();
                current_dir = info.0.as_ref().unwrap().clone();
            } else if line.starts_with("$ cd") {
                current_dir = line.split_whitespace().skip(2).next().unwrap().to_owned();                
            } else if line == "$ ls" {
                // Intentionally blank
            } else {
                let mut parts = line.split_whitespace();
                let info = parts.next().unwrap();
                let name = parts.next().unwrap().clone();
                if info == "dir" {
                    system.directories.insert(name.to_owned(), (Some(current_dir.clone()), vec![]));
                } else {
                    let size = info.parse::<usize>().unwrap();
                    system.files.insert(name.to_owned(), (current_dir.clone(), size));
                }
                let dir = system.directories.get_mut(current_dir.as_str()).unwrap();
                dir.1.push(name.to_owned());
            }
        }
        Ok(system)
    }

    pub fn size_of(&self, filename: &str) -> usize {
        match self.files.get(filename) {
            Some((_, size)) => *size,
            None => match self.directories.get(filename) {
                None => panic!("This shouldn't happen!"),
                Some((_,files)) => {
                    files.iter().map(|f| self.size_of(f)).sum()
                }
            }
        }
    }

    pub fn part1(&self) -> usize {
        self.directories.keys().map(|k| self.size_of(k)).filter(|s| *s <= 100000).sum()
    }
}
