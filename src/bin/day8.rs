use advent_code_lib::{simpler_main, DirType, GridWorld, ManhattanDir, Position};
use enum_iterator::*;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let map = GridWorld::from_file(filename)?;
        println!("Part 1: {}", part1(&map));
        println!("Part 2: {}", part2(&map));
        Ok(())
    })
}

fn part1(map: &GridWorld) -> usize {
    map.position_iter()
        .filter(|p| all::<ManhattanDir>().any(|dir| find_blocking_tree(map, *p, dir).is_none()))
        .count()
}

fn find_blocking_tree(map: &GridWorld, p: Position, dir: ManhattanDir) -> Option<Position> {
    let tree_height = map.value(p).unwrap();
    let mut p = dir.next_position(p);
    while let Some(blocker) = map.value(p) {
        if blocker >= tree_height {
            return Some(p);
        }
        p = dir.next_position(p);
    }
    None
}

fn part2(map: &GridWorld) -> usize {
    map.position_iter()
        .map(|p| scenic_score(map, p))
        .max()
        .unwrap()
}

fn scenic_score(map: &GridWorld, p: Position) -> usize {
    all::<ManhattanDir>()
        .map(|dir| {
            find_blocking_tree(map, p, dir).map_or_else(
                || match dir {
                    ManhattanDir::N => p.row as usize,
                    ManhattanDir::W => p.col as usize,
                    ManhattanDir::S => map.height() - p.row as usize,
                    ManhattanDir::E => map.width() - p.col as usize,
                },
                |b| p.manhattan_distance(b),
            )
        })
        .product()
}
