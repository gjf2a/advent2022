use std::{
    collections::{BTreeMap, BinaryHeap, BTreeSet},
    fmt::Display, iter::repeat,
};

use advent_code_lib::{
    all_lines, breadth_first_search, search, simpler_main, ContinueSearch, SearchQueue,
};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let tunnels = TunnelGraph::from_file(filename)?;
        println!("Part 1: {}", part1(&tunnels));
        println!("Part 2: {}", part2(&tunnels));
        Ok(())
    })
}

pub fn part1(tunnels: &TunnelGraph) -> usize {
    let start = PressureNode::start_at(tunnels, 30, false);
    conduct_search(tunnels, start)
}

pub fn part2(tunnels: &TunnelGraph) -> usize {
    let start = PressureNode::start_at(tunnels, 26, true);
    conduct_search(tunnels, start)
}

fn conduct_search(tunnels: &TunnelGraph, start: PressureNode) -> usize {
    let mut best = 0;
    let mut visited = BTreeSet::new();
    let mut queue = PressureQueue::new();
    queue.enqueue(&start);

    let result = search(queue, |s, q| {
        let options = s.successors(tunnels);
        let potential = potential(tunnels, s.min_minutes_left(), &options);
        if s.total_pressure + potential >= best {
            for i in 0..s.explorers.len() {
                for successor in options.iter() {
                    if let Some(node) = s.successor(i, *successor, tunnels) {
                        if !visited.contains(&node) {
                            visited.insert(node.clone());
                            if node.total_pressure > best {
                                best = node.total_pressure;
                            }
                            q.enqueue(&node);
                        }
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

fn potential(tunnels: &TunnelGraph, minutes_left: usize, remaining_nodes: &Vec<usize>) -> usize {
    let mut values: Vec<usize> = remaining_nodes
        .iter()
        .map(|n| tunnels.pressure_for(*n))
        .collect();
    values.sort_by(|a, b| b.cmp(a));
    while values.len() > minutes_left {
        values.pop();
    }
    values.iter().sum::<usize>() * minutes_left
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ExplorerState {
    minutes_left: usize,
    at: usize,
}

#[derive(Default, Clone, Eq, PartialEq, Ord, Debug)]
struct PressureNode {
    explorers: Vec<ExplorerState>,
    opened: Vec<usize>,
    total_pressure: usize,
}

impl PressureNode {
    fn start_at(tunnels: &TunnelGraph, minutes_left: usize, elephant_help: bool) -> Self {
        let start = tunnels.start_valve();
        let mut result = Self::default();
        result.explorers.push(ExplorerState {
            at: start,
            minutes_left,
        });
        if elephant_help {
            result.explorers.push(ExplorerState {
                at: start,
                minutes_left,
            });
        }
        result.opened = repeat(0).take(tunnels.names.len()).collect();
        result
    }

    fn min_minutes_left(&self) -> usize {
        self.explorers.iter().map(|ex| ex.minutes_left).min().unwrap()
    }

    fn successor(&self, explorer: usize, valve: usize, tunnels: &TunnelGraph) -> Option<PressureNode> {
        let moves = tunnels.valve_activation_times[self.explorers[explorer].at][valve];
        if moves <= self.explorers[explorer].minutes_left {
            let mut opened = self.opened.clone();
            let pressure_from = (self.explorers[explorer].minutes_left - moves) * tunnels.pressure_for(valve);
            opened[valve] = pressure_from;
            let mut updated_explorers = self.explorers.clone();
            updated_explorers[explorer].at = valve;
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

    fn successors(&self, tunnels: &TunnelGraph) -> Vec<usize> {
        tunnels
            .valves()
            .filter(|valve| self.opened[*valve] == 0 && tunnels.pressure_for(*valve) > 0)
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
    names: Vec<String>,
    ids: BTreeMap<String, usize>,
    valve2flow: Vec<usize>,
    valve2tunnels: Vec<Vec<usize>>,
    valve_activation_times: Vec<Vec<usize>>,
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
        let mut tunnel_list = Vec::new();
        for line in all_lines(filename)? {
            let mut parts = line.split_whitespace();
            let name = parts.by_ref().skip(1).next().unwrap();
            let rate = parse_rate(parts.by_ref().skip(2).next().unwrap());
            let tunnels: Vec<String> = parts.by_ref().skip(4).map(|s| s[..2].to_string()).collect();
            
            let id_num = result.names.len();
            result.ids.insert(name.to_string(), id_num);
            result.names.push(name.to_string());
            result.valve2flow.push(rate);
            tunnel_list.push(tunnels);
        }
        for tunnels in tunnel_list {
            let tunnels = tunnels.iter().map(|s| result.ids.get(s).copied().unwrap()).collect();
            result.valve2tunnels.push(tunnels);
        }
        result.valve_activation_times = result
            .valves()
            .map(|valve| result.activation_times_from(valve))
            .collect();
        Ok(result)
    }

    pub fn start_valve(&self) -> usize {
        self.ids.get("AA").copied().unwrap()
    }

    pub fn pressure_for(&self, valve: usize) -> usize {
        self.valve2flow[valve]
    }

    pub fn valves(&self) -> impl Iterator<Item = usize> {
        0..self.valve2flow.len()
    }

    fn activation_times_from(&self, src: usize) -> Vec<usize> {
        let parents = breadth_first_search(&src, |s, q| {
            for neighbor in self.valve2tunnels[*s].iter() {
                q.enqueue(neighbor);
            }
            ContinueSearch::Yes
        });

        self.valves()
            .filter_map(|valve| parents.path_back_from(&valve).map(|p| p.len()))
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
            tunnels.activation_times_from(tunnels.start_valve()),
            vec![1, 2, 3, 2, 3, 4, 5, 6, 2, 3]
        );
    }
}
