use std::collections::BTreeMap;

use advent_code_lib::{simpler_main, all_lines};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| { 
        let system = FileSystem::from_file(filename)?;
        println!("Part 1: {}", system.part1());
        println!("Part 2: {}", system.part2());
        Ok(())
    })
}

#[derive(Debug, Default)]
pub struct FileSystem {
    next_inode: usize,
    directories: BTreeMap<usize,(Option<usize>,BTreeMap<String,usize>)>,
    files: BTreeMap<usize,usize>
}

impl FileSystem {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut system = FileSystem::default();
        let mut current_dir = system.new_directory(None);
        for line in all_lines(filename)? {
            if line == "$ cd .." {
                let (parent, _) = system.directories.get(&current_dir).unwrap();
                current_dir = parent.unwrap();
            } else if line == "$ cd /" {
                current_dir = 0;
            } else if line.starts_with("$ cd") {
                let dir_name = line.split_whitespace().skip(2).next().unwrap();                
                current_dir = *system.directories.get(&current_dir).unwrap().1.get(dir_name).unwrap();
            } else if line == "$ ls" {
                // Intentionally blank
            } else {
                let mut parts = line.split_whitespace();
                let info = parts.next().unwrap();
                let name = parts.next().unwrap().clone();
                let id = if info == "dir" {
                    system.new_directory(Some(current_dir))
                } else {
                    system.new_file(info.parse::<usize>().unwrap())
                };
                system.directories.get_mut(&current_dir).unwrap().1.insert(name.to_owned(), id);
            }
        }
        Ok(system)
    }

    fn new_directory(&mut self, parent: Option<usize>) -> usize {
        let id = self.new_id();
        self.directories.insert(id, (parent, BTreeMap::new()));
        id
    }

    fn new_file(&mut self, size: usize) -> usize {
        let id = self.new_id();
        self.files.insert(id, size);
        id
    }

    fn new_id(&mut self) -> usize {
        let id = self.next_inode;
        self.next_inode += 1;
        id
    }

    pub fn size_of(&self, id: usize) -> usize {
        match self.files.get(&id) {
            Some(size) => *size,
            None => match self.directories.get(&id) {
                None => panic!("This shouldn't happen!"),
                Some((_,files)) => {
                    files.iter().map(|(_,id)| self.size_of(*id)).sum()
                }
            }
        }
    }

    pub fn part1(&self) -> usize {
        self.directories.keys().map(|k| self.size_of(*k)).filter(|s| *s <= 100000).sum()
    }

    pub fn part2(&self) -> usize {
        const TOTAL_SPACE: usize = 70000000;
        const SPACE_NEEDED: usize = 30000000;
        let mut sizes = self.directories.keys()
            .map(|k| self.size_of(*k));
        let space_available = TOTAL_SPACE - sizes.next().unwrap();
        sizes.filter(|s| space_available + *s >= SPACE_NEEDED)
            .min()
            .unwrap()
    }
}
