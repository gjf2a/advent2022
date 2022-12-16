use std::collections::{BTreeMap, BinaryHeap};

use advent_code_lib::{
    all_lines, breadth_first_search, search, simpler_main, ContinueSearch, SearchQueue,
};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let tunnels = TunnelGraph::from_file(filename)?;
        println!("Part 1: {}", part1(&tunnels));
        Ok(())
    })
}

pub fn part1(tunnels: &TunnelGraph) -> usize {
    let mut best = 0;
    let mut queue = PressureQueue::new();
    let start = PressureNode::start_at(tunnels.start.as_str(), 30);
    queue.enqueue(&start);

    search(queue, |s, q| {
        let options = s.successors(tunnels);
        let potential: usize = options
            .iter()
            .map(|opt| tunnels.pressure_for(opt) * s.minutes_left)
            .sum();
        if s.total_pressure + potential >= best {
            for successor in options.iter() {
                if let Some(node) = s.successor(successor.as_str(), tunnels) {
                    if node.total_pressure > best {
                        best = node.total_pressure;
                    }
                    q.enqueue(&node);
                }
            }
        }
        ContinueSearch::Yes
    });
    best
}

#[derive(Default, Clone, Eq, PartialEq, Ord, Debug)]
struct PressureNode {
    at: String,
    minutes_left: usize,
    opened: BTreeMap<String, usize>,
    total_pressure: usize,
}

impl PressureNode {
    fn start_at(start: &str, minutes: usize) -> Self {
        let mut result = Self::default();
        result.at = start.to_owned();
        result.minutes_left = minutes;
        result
    }

    fn successor(&self, valve: &str, tunnels: &TunnelGraph) -> Option<PressureNode> {
        let moves = tunnels
            .valve_activation_times
            .get(&self.at)
            .and_then(|m| m.get(valve))
            .copied()
            .unwrap();
        if moves <= self.minutes_left {
            let mut opened = self.opened.clone();
            let pressure_from = (self.minutes_left - moves) * tunnels.pressure_for(valve);
            opened.insert(valve.to_string(), pressure_from);
            Some(PressureNode {
                at: valve.to_string(),
                minutes_left: self.minutes_left - moves,
                opened,
                total_pressure: self.total_pressure + pressure_from,
            })
        } else {
            None
        }
    }

    fn successors(&self, tunnels: &TunnelGraph) -> Vec<String> {
        tunnels
            .valves()
            .filter(|valve| !self.opened.contains_key(*valve))
            .cloned()
            .collect()
    }
}

impl PartialOrd for PressureNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.total_pressure.partial_cmp(&other.total_pressure) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minutes_left.partial_cmp(&other.minutes_left)
    }
}

#[derive(Default)]
struct PressureQueue {
    heap: BinaryHeap<PressureNode>,
}

impl SearchQueue<PressureNode> for PressureQueue {
    fn new() -> Self {
        Self::default()
    }

    fn enqueue(&mut self, item: &PressureNode) {
        self.heap.push(item.clone());
    }

    fn dequeue(&mut self) -> Option<PressureNode> {
        self.heap.pop()
    }

    fn len(&self) -> usize {
        self.heap.len()
    }
}

#[derive(Default, Clone, Debug)]
pub struct TunnelGraph {
    start: String,
    valve2flow: BTreeMap<String, usize>,
    valve2tunnels: BTreeMap<String, Vec<String>>,
    valve_activation_times: BTreeMap<String, BTreeMap<String, usize>>,
}

fn parse_rate(rate: &str) -> usize {
    rate.split('=')
        .skip(1)
        .next()
        .unwrap()
        .split(';')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

impl TunnelGraph {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut result = TunnelGraph::default();
        for line in all_lines(filename)? {
            let mut parts = line.split_whitespace();
            let name = parts.by_ref().skip(1).next().unwrap();
            if result.start.len() == 0 {
                result.start = name.to_string();
            }
            let rate = parse_rate(parts.by_ref().skip(2).next().unwrap());
            let tunnels = parts.by_ref().skip(4).map(|s| s[..2].to_string()).collect();
            result.valve2flow.insert(name.to_string(), rate);
            result.valve2tunnels.insert(name.to_string(), tunnels);
        }
        result.valve_activation_times = result
            .valves()
            .map(|valve| (valve.clone(), result.activation_times_from(valve)))
            .collect();
        Ok(result)
    }

    pub fn pressure_for(&self, valve: &str) -> usize {
        self.valve2flow.get(valve).copied().unwrap()
    }

    pub fn valves(&self) -> impl Iterator<Item = &String> {
        self.valve2flow.keys()
    }

    fn activation_times_from(&self, src: &str) -> BTreeMap<String, usize> {
        let parents = breadth_first_search(&src.to_string(), |s, q| {
            for neighbor in self.valve2tunnels.get(s).unwrap() {
                q.enqueue(neighbor);
            }
            ContinueSearch::Yes
        });

        self.valves()
            .filter_map(|valve| parents.path_back_from(valve).map(|p| (valve.clone(), p)))
            .map(|(v, p)| (v, p.len()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::TunnelGraph;

    #[test]
    pub fn test() {
        let tunnels = TunnelGraph::from_file("ex/day16.txt").unwrap();
        assert_eq!(
            format!("{:?}", tunnels.activation_times_from("AA")),
            r#"{"AA": 1, "BB": 2, "CC": 3, "DD": 2, "EE": 3, "FF": 4, "GG": 5, "HH": 6, "II": 2, "JJ": 3}"#
        );
    }
}
