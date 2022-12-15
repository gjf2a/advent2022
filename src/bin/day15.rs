use std::{collections::{BTreeSet}};

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
    manhattan_radius: usize,
}

impl ManhattanNeighborhood {
    pub fn from(sensor: Position, closest_beacon: Position) -> Self {
        Self {sensor, closest_beacon, manhattan_radius: sensor.manhattan_distance(closest_beacon)}
    }

    pub fn contains(&self, p: Position) -> bool {
        self.sensor.manhattan_distance(p) <= self.manhattan_radius
    }

    pub fn num_in_row(&self, row: isize) -> usize {
        match self.column_x_range_for(row) {
            None => 0,
            Some((x_start, x_end)) => {
                let mut result = (x_end - x_start + 1) as usize;
                if row == self.sensor.row {
                    result -= 1;
                }
                if row == self.closest_beacon.row {
                    result -= 1;
                }
                result
            }
        }
    }

    pub fn column_x_range_for(&self, row: isize) -> Option<(isize, isize)> {
        let row_diff = (self.sensor.row - row).abs() as usize;
        if self.manhattan_radius < row_diff {
            None
        } else {
            let offset = self.manhattan_radius - row_diff;
            Some((self.sensor.col - offset as isize, self.sensor.col + offset as isize))
        }
    }

    pub fn overlaps_in_row(&self, other: Self, row: isize) -> usize { 
        if let Some((self_x_start, self_x_end)) = self.column_x_range_for(row) {
            if let Some((other_x_start, other_x_end)) = other.column_x_range_for(row) {
                if self_x_start > other_x_start {
                    other.overlaps_in_row(*self, row)
                } else if self_x_end < other_x_start {
                    0
                } else if self_x_end > other_x_end {
                    other.num_in_row(row)
                } else {
                    let mut result = (self_x_end - other_x_start + 1) as usize;
                    //println!("result: {result}");
                    let objects = [self.closest_beacon, other.closest_beacon, self.sensor, other.sensor];
                    let objects = objects.iter().collect::<BTreeSet<_>>();
                    for object in objects.iter() {
                        if object.row == row && (other_x_start..=self_x_end).contains(&object.col) {
                            //println!("beacon: {beacon}");
                            result -= 1;
                        }
                    }
                    result
                }
            } else {
                0
            }
        } else {
            0
        }
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

    pub fn num_no_beacon(&self, row: isize) -> usize {
        let mut big2small: Vec<ManhattanNeighborhood> = self.sensors.iter().copied().collect();
        big2small.sort_by(|n1, n2| n2.num_in_row(row).cmp(&n1.num_in_row(row)));
        let mut total = 0;
        for i in 0..big2small.len() {
            let n_i = big2small[i];
            if n_i.column_x_range_for(row).is_some() {
                println!("Adding {} ({:?}) {n_i:?}", n_i.num_in_row(row), n_i.column_x_range_for(row));
                total += n_i.num_in_row(row);
                for j in i+1..self.sensors.len() {
                    let n_j = big2small[j];
                    if n_i.overlaps_in_row(n_j, row) > 0 {
                        println!("Subtracting {} ({:?}) {n_j:?}", n_i.overlaps_in_row(n_j, row), n_j.column_x_range_for(row));
                        total -= n_i.overlaps_in_row(n_j, row);
                    }
                }
            }
        }
        total
    }
}