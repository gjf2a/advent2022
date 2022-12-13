use std::cmp::min;

use advent_code_lib::{
    breadth_first_search, simpler_main, ContinueSearch, GridCharWorld, ParentMap, Position,
    SearchQueue,
};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let map = GridCharWorld::from_char_file(filename)?;
        println!("Part 1: {}", part1(&map));
        println!("Part 2: {}", part2(&map));
        Ok(())
    })
}

fn height_of(c: char) -> char {
    match c {
        'S' => 'a',
        'E' => 'z',
        _ => c,
    }
}

fn part1(map: &GridCharWorld) -> usize {
    distance_to_goal_from(map, map.any_position_for('S')).unwrap()
}

fn climb_distance(start: char, end: char) -> i8 {
    end as i8 - start as i8
}

fn distance_to_goal_from(map: &GridCharWorld, start: Position) -> Option<usize> {
    let end = map.any_position_for('E');
    let parents: ParentMap<Position> = breadth_first_search(&start, |p, q| {
        let p_height = height_of(map.value(*p).unwrap());
        for neighbor in p.manhattan_neighbors() {
            let distance = map
                .value(neighbor)
                .map(|h| climb_distance(p_height, height_of(h)));
            if let Some(distance) = distance {
                if distance <= 1 {
                    q.enqueue(&neighbor);
                }
            }
        }
        ContinueSearch::Yes
    });
    parents
        .path_back_from(&end)
        .map(|path_back| path_back.len() - 1)
}

fn part2(map: &GridCharWorld) -> usize {
    min(
        part1(map),
        map.positions_for('a')
            .iter()
            .filter_map(|start| distance_to_goal_from(map, *start))
            .min()
            .unwrap(),
    )
}
