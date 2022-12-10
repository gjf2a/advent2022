use advent_code_lib::{simpler_main, all_lines};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let mut cpu = Cpu::new();
        for line in all_lines(filename)? {
            cpu.instruction(line.as_str());
        }
        let part1: i64 = (20..=220).step_by(40).map(|cycle| cpu.strength_during(cycle)).sum();
        println!("Part 1: {part1}");
        Ok(())
    })
}

pub struct Cpu {
    x: i64,
    completed_cycles: usize,
    signal_strengths: Vec<i64>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {x: 1, completed_cycles: 0, signal_strengths: vec![]}
    }

    pub fn instruction(&mut self, instruction: &str) {
        self.add_signal_strength();
        let mut parts = instruction.split_whitespace();
        let opcode = parts.next().unwrap();
        match opcode {
            "noop" => {}
            "addx" => {
                self.add_signal_strength();
                self.x += parts.next().unwrap().parse::<i64>().unwrap();
            }
            _ => panic!("Did not recognize {opcode}")
        }
    }

    pub fn add_signal_strength(&mut self) {
        self.completed_cycles += 1;
        self.signal_strengths.push(self.completed_cycles as i64 * self.x);
    }

    pub fn strength_during(&self, cycle: usize) -> i64 {
        self.signal_strengths[cycle - 1]
    }
}