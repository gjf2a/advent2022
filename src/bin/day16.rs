use std::{
    collections::{BTreeMap, BinaryHeap},
    fmt::Display,
};

use advent_code_lib::{
    all_lines, breadth_first_search, search, simpler_main, ContinueSearch, SearchQueue,
};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let tunnels = TunnelGraph::from_file(filename)?;
        println!("Part 1: {}", part1(&tunnels));
        println!("Part 1: {}", part2(&tunnels));
        Ok(())
    })
}

pub fn part1(tunnels: &TunnelGraph) -> usize {
    let start = PressureNode::start_at("AA", 30, false);
    conduct_search(tunnels, start)
}

pub fn part2(tunnels: &TunnelGraph) -> usize {
    let start = PressureNode::start_at("AA", 26, true);
    conduct_search(tunnels, start)
}

fn conduct_search(tunnels: &TunnelGraph, start: PressureNode) -> usize {
    let mut best = 0;
    let mut queue = PressureQueue::new();
    queue.enqueue(&start);

    let result = search(queue, |s, q| {
        let options = s.successors(tunnels);
        let potential = potential(tunnels, s.min_minutes_left(), &options);
        if s.total_pressure + potential >= best {
            for i in 0..s.explorers.len() {
                for successor in options.iter() {
                    if let Some(node) = s.successor(i, successor.as_str(), tunnels) {
                        if node.total_pressure > best {
                            best = node.total_pressure;
                        }
                        q.enqueue(&node);
                    }
                }
            }
        }
        ContinueSearch::Yes
    });
    println!(
        "enqueued: {} (dequeued {})",
        result.enqueued(),
        result.dequeued()
    );
    best
}

fn potential(tunnels: &TunnelGraph, minutes_left: usize, remaining_nodes: &Vec<String>) -> usize {
    let mut values: Vec<usize> = remaining_nodes
        .iter()
        .map(|n| tunnels.pressure_for(n.as_str()))
        .collect();
    values.sort_by(|a, b| b.cmp(a));
    while values.len() > minutes_left {
        values.pop();
    }
    values.iter().sum::<usize>() * minutes_left
}

/*fn potential2(tunnels: &TunnelGraph, minutes_left: usize, remaining_nodes: &Vec<String>) -> usize {
    0
}*/

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ExplorerState {
    minutes_left: usize,
    at: String,
}

#[derive(Default, Clone, Eq, PartialEq, Ord, Debug)]
struct PressureNode {
    explorers: Vec<ExplorerState>,
    opened: BTreeMap<String, usize>,
    total_pressure: usize,
}

impl PressureNode {
    fn start_at(start: &str, minutes_left: usize, elephant_help: bool) -> Self {
        let mut result = Self::default();
        result.explorers.push(ExplorerState {
            at: start.to_owned(),
            minutes_left,
        });
        if elephant_help {
            result.explorers.push(ExplorerState {
                at: start.to_owned(),
                minutes_left,
            });
        }
        result
    }

    fn min_minutes_left(&self) -> usize {
        self.explorers.iter().map(|ex| ex.minutes_left).min().unwrap()
    }

    fn successor(&self, explorer: usize, valve: &str, tunnels: &TunnelGraph) -> Option<PressureNode> {
        let moves = tunnels
            .valve_activation_times
            .get(&self.explorers[explorer].at)
            .and_then(|m| m.get(valve))
            .copied()
            .unwrap();
        if moves <= self.explorers[explorer].minutes_left {
            let mut opened = self.opened.clone();
            let pressure_from = (self.explorers[explorer].minutes_left - moves) * tunnels.pressure_for(valve);
            opened.insert(valve.to_string(), pressure_from);
            let mut updated_explorers = self.explorers.clone();
            updated_explorers[explorer].at = valve.to_string();
            updated_explorers[explorer].minutes_left -= moves;
            Some(PressureNode {
                explorers: updated_explorers,
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
            .filter(|valve| !self.opened.contains_key(*valve) && tunnels.pressure_for(*valve) > 0)
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
        self.explorers.partial_cmp(&other.explorers)
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
    valve2flow: BTreeMap<String, usize>,
    valve2tunnels: BTreeMap<String, Vec<String>>,
    valve_activation_times: BTreeMap<String, BTreeMap<String, usize>>,
}

impl Display for TunnelGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for valve in self.valves() {
            let out = format!("{:?}", self.valve2tunnels.get(valve).unwrap())
                .replace("[", "")
                .replace("]", "")
                .replace('"', "");
            write!(
                f,
                "Valve {} has flow rate={}; tunnels lead to valves {}\n",
                valve,
                self.pressure_for(valve),
                out
            )?
        }
        Ok(())
    }
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
