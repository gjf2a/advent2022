use std::{
    cmp::{max, min},
    collections::{VecDeque, BTreeSet},
};

use advent_code_lib::{all_lines, all_positions_from, simpler_main, Position};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let part_1_row = if filename.contains("ex") { 10 } else { 2000000 };
        let map = BeaconMap::from_file(filename)?;
        println!("Part 1: {}", map.num_no_beacon(part_1_row));
        let part_2_dimension = if filename.contains("ex") { 20 } else { 4000000 };
        let (x, y) = map.find_beacon(part_2_dimension);
        println!("Location: ({x}, {y})");
        Ok(())
    })
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Dim {
    Col,
    Row,
}

impl Dim {
    pub fn inv(&self, p: Position) -> isize {
        match self {
            Self::Col => Self::Row.get(p),
            Self::Row => Self::Col.get(p),
        }
    }

    pub fn get(&self, p: Position) -> isize {
        match self {
            Self::Col => p.col,
            Self::Row => p.row,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct ManhattanNeighborhood {
    sensor: Position,
    closest_beacon: Position,
    manhattan_radius: isize,
}

impl ManhattanNeighborhood {
    pub fn from(sensor: Position, closest_beacon: Position) -> Self {
        Self {
            sensor,
            closest_beacon,
            manhattan_radius: sensor.manhattan_distance(closest_beacon) as isize,
        }
    }
    
    pub fn contains(&self, x: isize, y: isize) -> bool {
        self.sensor.manhattan_distance(Position::from((x, y))) as isize <= self.manhattan_radius
    }

    pub fn range_for(&self, d: Dim, i: isize) -> Option<Range> {
        let diff = (d.get(self.sensor) - i).abs();
        let offset = self.manhattan_radius - diff;
        Range::new(d.inv(self.sensor) - offset, d.inv(self.sensor) + offset)
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ManhattanRange {
    column: Range,
    row: Range,
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Range {
    start: isize,
    end: isize,
}

impl Range {
    pub fn new(start: isize, end: isize) -> Option<Self> {
        if start <= end {
            Some(Self { start, end })
        } else {
            None
        }
    }

    pub fn expand(&self) -> Vec<isize> {
        (self.start..=self.end).collect()
    }

    pub fn contains(&self, value: isize) -> bool {
        (self.start..=self.end).contains(&value)
    }

    pub fn count(&self) -> isize {
        self.end - self.start + 1
    }

    pub fn overlaps_with(&self, other: &Range) -> bool {
        self.contains(other.start)
            || self.contains(other.end)
            || other.contains(self.start)
            || other.contains(self.end)
    }

    pub fn absorb(&mut self, other: &Range) {
        assert!(self.overlaps_with(other));
        self.start = min(self.start, other.start);
        self.end = max(self.end, other.end);
    }

    pub fn split_as_needed(ranges: &Vec<Range>, value: isize) -> Vec<Range> {
        let mut result = vec![];
        for range in ranges.iter() {
            if range.contains(value) {
                if let Some(r) = Range::new(range.start, value - 1) {
                    result.push(r);
                }
                if let Some(r) = Range::new(value + 1, range.end) {
                    result.push(r);
                }
            } else {
                result.push(*range);
            }
        }
        result
    }
}

#[derive(Default, Clone, Debug)]
pub struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    pub fn add_range(&mut self, new_range: Range) {
        self.ranges.push(new_range);
        self.ranges.sort();
        let mut old_ranges = VecDeque::new();
        loop {
            match self.ranges.pop() {
                None => break,
                Some(popped) => old_ranges.push_front(popped),
            }
        }

        let mut current = old_ranges.pop_front().unwrap();
        loop {
            match old_ranges.pop_front() {
                Some(mut popped) => {
                    if current.overlaps_with(&popped) {
                        current.absorb(&popped);
                    } else {
                        std::mem::swap(&mut popped, &mut current);
                        self.ranges.push(popped);
                    }
                }
                None => {
                    self.ranges.push(current);
                    return;
                }
            }
        }
    }

    pub fn count(&self) -> isize {
        let mut total = 0;
        for range in self.ranges.iter() {
            total += range.count();
        }
        total
    }

    pub fn split_as_needed(&mut self, value: isize) {
        self.ranges = Range::split_as_needed(&mut self.ranges, value);
    }

    pub fn uncovered_within(&self, limit: isize) -> Vec<isize> {
        let mut result = vec![];
        let mut uncovered = Ranges {ranges: vec![Range::new(0, limit).unwrap()]};
        for range in self.ranges.iter() {
            uncovered.remove_from(range);
        }
        for range in uncovered.ranges.iter() {
            let mut is = range.expand();
            result.append(&mut is);
        }
        result
    }

    pub fn remove_from(&mut self, other: &Range) {
        
    }
}

#[derive(Clone, Default, Debug)]
pub struct BeaconMap {
    sensors: Vec<ManhattanNeighborhood>,
}

impl BeaconMap {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut result = Self::default();
        for line in all_lines(filename)? {
            let mut positions = all_positions_from(line);
            let sensor = positions.pop_front().unwrap();
            let beacon = positions.pop_front().unwrap();
            result.add_sensor_beacon(sensor, beacon);
        }
        Ok(result)
    }

    fn add_sensor_beacon(&mut self, sensor: Position, beacon: Position) {
        self.sensors
            .push(ManhattanNeighborhood::from(sensor, beacon));
    }

    pub fn coverage(&self, d: Dim, i: isize) -> Ranges {
        let mut ranges = Ranges::default();
        for sensor in self.sensors.iter() {
            if let Some(r) = sensor.range_for(d, i) {
                ranges.add_range(r);
            }
        }
        for sensor in self.sensors.iter() {
            if d.get(sensor.sensor) == i {
                ranges.split_as_needed(sensor.sensor.col);
            }
            if d.get(sensor.closest_beacon) == i {
                ranges.split_as_needed(sensor.closest_beacon.col);
            }
        }
        ranges
    }

    pub fn num_no_beacon(&self, row: isize) -> isize {
        self.coverage(Dim::Row, row).count()
    }

    fn find_with_gap(&self, d: Dim, limit: isize) -> Vec<isize> {
        let mut result = vec![];
        for i in 0..=limit {
            let r = self.coverage(d, i);
            println!("{r:?}");
            let mut found = r.uncovered_within(limit);
            println!("{found:?}");
            result.append(&mut found);
        }
        return result;
    }

    pub fn find_beacon(&self, limit: isize) -> (isize, isize) {
        let xs = self.find_with_gap(Dim::Col, limit);
        let ys = self.find_with_gap(Dim::Row, limit);
        let mut candidates = BTreeSet::new();
        for x in xs.iter() {
            for y in ys.iter() {
                candidates.insert((*x, *y));
            }
        }
        println!("{candidates:?}");
        for sensor in self.sensors.iter() {
            candidates.retain(|c| !sensor.contains(c.0, c.1));
        }
        assert_eq!(1, candidates.len());
        candidates.iter().next().copied().unwrap()
    }
}
