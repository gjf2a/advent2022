use std::cmp::{min, max};

use advent_code_lib::{all_lines, simpler_main, all_positions_from, Position};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let part_1_row = if filename.contains("ex") {10} else {2000000};
        let map = BeaconMap::from_file(filename)?;
        println!("Part 1: {}", map.num_no_beacon(part_1_row));
        Ok(())
    })
}

#[derive(Clone, Copy, Default, Debug)]
pub struct ManhattanNeighborhood {
    sensor: Position,
    closest_beacon: Position,
    manhattan_radius: isize,
}

impl ManhattanNeighborhood {
    pub fn from(sensor: Position, closest_beacon: Position) -> Self {
        Self {sensor, closest_beacon, manhattan_radius: sensor.manhattan_distance(closest_beacon) as isize}
    }

    pub fn row_ranges(&self, row: isize) -> Vec<Range> {
        let mut result = vec![];
        let row_diff = (self.sensor.row - row).abs();
        let offset = self.manhattan_radius - row_diff;
        if let Some(range) = Range::new(self.sensor.col - offset, self.sensor.col + offset) {
            result.push(range);
            if row == self.sensor.row {
                result = Range::split_as_needed(&result, self.sensor.col);
            }
            if row == self.closest_beacon.row {
                result = Range::split_as_needed(&result, self.closest_beacon.col);
            }
        }
        result
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Range {
    start: isize,
    end: isize
}

impl Range {
    pub fn new(start: isize, end: isize) -> Option<Self> {
        if start <= end {
            Some(Self {start, end})
        } else {
            None
        }
    }

    pub fn contains(&self, value: isize) -> bool {
        (self.start..=self.end).contains(&value)
    }

    pub fn count(&self) -> isize {
        self.end - self.start + 1
    }

    pub fn overlaps_with(&self, other: &Range) -> bool {
        self.contains(other.start) || self.contains(other.end) || other.contains(self.start) || other.contains(self.end) 
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
    ranges: Vec<Range>
}

impl Ranges {
    pub fn add_range(&mut self, new_range: Range) {
        for range in self.ranges.iter_mut() {
            if range.overlaps_with(&new_range) {
                range.absorb(&new_range);
                return;
            }
        }
        self.ranges.push(new_range)
    }

    pub fn count(&self) -> isize {
        let mut total = 0;
        for range in self.ranges.iter() {
            total += range.count();
        }
        total
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
        self.sensors.push(ManhattanNeighborhood::from(sensor, beacon)); 
    }

    pub fn num_no_beacon(&self, row: isize) -> isize {
        let mut ranges = Ranges::default();
        for sensor in self.sensors.iter() {
            for r in sensor.row_ranges(row) {
                println!("{r:?}");
                ranges.add_range(r);
            }
        }
        ranges.ranges.sort();
        println!("{ranges:?}");
        ranges.count()
    }
}