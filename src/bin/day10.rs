use advent_code_lib::{all_lines, simpler_main};

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| {
        let mut cpu = Cpu::new();
        for line in all_lines(filename)? {
            cpu.instruction(line.as_str());
        }
        let part1: i64 = (20..=220)
            .step_by(40)
            .map(|cycle| cpu.strength_during(cycle))
            .sum();
        println!("Part 1: {part1}");
        cpu.render();
        Ok(())
    })
}

pub struct Cpu {
    x: i64,
    completed_cycles: usize,
    signal_strengths: Vec<i64>,
    x_values: Vec<i64>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            x: 1,
            completed_cycles: 0,
            signal_strengths: vec![],
            x_values: vec![],
        }
    }

    pub fn instruction(&mut self, instruction: &str) {
        self.record_state();
        let mut parts = instruction.split_whitespace();
        let opcode = parts.next().unwrap();
        match opcode {
            "noop" => {}
            "addx" => {
                self.record_state();
                self.x += parts.next().unwrap().parse::<i64>().unwrap();
            }
            _ => panic!("Did not recognize {opcode}"),
        }
    }

    pub fn record_state(&mut self) {
        self.completed_cycles += 1;
        self.signal_strengths
            .push(self.completed_cycles as i64 * self.x);
        self.x_values.push(self.x);
    }

    pub fn strength_during(&self, cycle: usize) -> i64 {
        self.signal_strengths[cycle - 1]
    }

    pub fn render(&self) {
        let on: Vec<bool> = self
            .x_values
            .iter()
            .copied()
            .enumerate()
            .map(|(i, x)| {
                let crt_x = (i % 40) as i64;
                (x - 1..=x + 1).contains(&crt_x)
            })
            .collect();
        for (i, b) in on.iter().enumerate() {
            if i % 40 == 0 {
                println!();
            }
            print!("{}", if *b { "#" } else { "." });
        }
        println!();
    }
}
