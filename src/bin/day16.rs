use std::collections::BTreeMap;

use advent_code_lib::{simpler_main, all_lines};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let tunnels = TunnelGraph::from_file(filename)?;
        println!("{tunnels:?}");
        Ok(())
    })
}

#[derive(Default, Clone, Debug)]
pub struct TunnelGraph {
    valve2flow: BTreeMap<String,u64>,
    valve2tunnels: BTreeMap<String,Vec<String>>,
}

impl TunnelGraph {
    pub fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut result = TunnelGraph::default();
        for line in all_lines(filename)? {
            let mut parts = line.split_whitespace();
            let name = parts.by_ref().skip(1).next().unwrap();
            let rate = parts.by_ref().skip(2).next().unwrap().split('=').skip(1).next().unwrap().split(';').next().unwrap().parse().unwrap();
            let tunnels = parts.by_ref().skip(4).map(|s| s[..2].to_string()).collect();
            result.valve2flow.insert(name.to_string(), rate);
            result.valve2tunnels.insert(name.to_string(), tunnels);
        }
        Ok(result)
    }
}