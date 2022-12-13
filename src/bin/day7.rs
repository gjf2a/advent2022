use std::collections::BTreeMap;

use advent_code_lib::{all_lines, simpler_main};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let system = FileSystem::from_file(filename)?;
        println!("Part 1: {}", system.part1());
        println!("Part 2: {}", system.part2());
        Ok(())
    })
}

#[derive(Debug, Clone)]
pub enum FileEntry {
    File(usize),
    Directory(Option<usize>, BTreeMap<String, usize>),
}

impl FileEntry {
    pub fn is_directory(&self) -> bool {
        match self {
            FileEntry::File(_) => false,
            FileEntry::Directory(_, _) => true,
        }
    }

    pub fn size(&self, system: &FileSystem) -> usize {
        match self {
            FileEntry::File(size) => *size,
            FileEntry::Directory(_, children) => children
                .values()
                .map(|i| system.inode2object[*i].size(system))
                .sum(),
        }
    }
}

#[derive(Debug, Default)]
pub struct FileSystem {
    inode2object: Vec<FileEntry>,
}

impl FileSystem {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut system = FileSystem::default();
        let mut current_dir = system.new_directory(None);
        for line in all_lines(filename)? {
            if line == "$ cd .." {
                if let FileEntry::Directory(parent, _) = system.inode2object[current_dir] {
                    if let Some(parent) = parent {
                        current_dir = parent;
                    }
                }
            } else if line == "$ cd /" {
                current_dir = 0;
            } else if line.starts_with("$ cd") {
                if let FileEntry::Directory(_, children) = &system.inode2object[current_dir] {
                    let dir_name = line.split_whitespace().skip(2).next().unwrap();
                    current_dir = children.get(dir_name).copied().unwrap();
                }
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
                if let FileEntry::Directory(_, children) = &mut system.inode2object[current_dir] {
                    children.insert(name.to_owned(), id);
                }
            }
        }
        Ok(system)
    }

    fn new_directory(&mut self, parent: Option<usize>) -> usize {
        let id = self.inode2object.len();
        self.inode2object
            .push(FileEntry::Directory(parent, BTreeMap::new()));
        id
    }

    fn new_file(&mut self, size: usize) -> usize {
        let id = self.inode2object.len();
        self.inode2object.push(FileEntry::File(size));
        id
    }

    pub fn directory_sizes(&self) -> Vec<usize> {
        self.inode2object
            .iter()
            .filter(|entry| entry.is_directory())
            .map(|dir| dir.size(self))
            .collect()
    }

    pub fn part1(&self) -> usize {
        let sizes = self.directory_sizes();
        sizes.iter().filter(|s| **s <= 100000).sum()
    }

    pub fn part2(&self) -> usize {
        const TOTAL_SPACE: usize = 70000000;
        const SPACE_NEEDED: usize = 30000000;
        let sizes = self.directory_sizes();
        let space_available = TOTAL_SPACE - sizes[0];
        sizes
            .iter()
            .skip(1)
            .filter(|s| space_available + *s >= SPACE_NEEDED)
            .min()
            .copied()
            .unwrap()
    }
}
