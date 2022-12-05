use advent_code_lib::{simpler_main, all_lines};
use enum_iterator::*;

fn main() -> anyhow::Result<()> {
    simpler_main(|filename| { 
        let mut part1_total = 0;
        let mut part2_total = 0;
        for line in all_lines(filename)? {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let opponent = Rps::from1(parts[0]);
            let me1 = Rps::from1(parts[1]);
            part1_total += me1.match_score(opponent);

            let me2 = opponent.strategy2(parts[1]);
            part2_total += me2.match_score(opponent);
        }
        println!("Part 1: {part1_total}");
        println!("Part 2: {part2_total}");
        Ok(())
    })
}

const WIN_SCORE: i64 = 6;
const LOSE_SCORE: i64 = 0;
const DRAW_SCORE: i64 = 3;

#[derive(Copy, Clone, PartialEq, Eq, Sequence, PartialOrd, Ord)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn from1(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Sorry!")
        }
    }

    fn move2match(&self, score: i64) -> Self {
        all::<Self>()
        .find(|me| me.game_score(*self) == score)
        .unwrap()
    }

    fn strategy2(&self, strategy: &str) -> Self {
        match strategy {
            "X" => self.move2match(LOSE_SCORE),
            "Y" => self.move2match(DRAW_SCORE),
            "Z" => self.move2match(WIN_SCORE),
            _ => panic!("sorry!")
        }
    }

    fn shape_score(&self) -> i64 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    fn defeats(&self, opponent: Self) -> bool {
        match (*self, opponent) {
            (Rps::Paper, Rps::Rock) | (Rps::Rock, Rps::Scissors) | (Rps::Scissors, Rps::Paper) => true,
            _ => false
        }
    }

    fn game_score(&self, opponent: Self) -> i64 {
        if self.defeats(opponent) {
            WIN_SCORE
        } else if opponent.defeats(*self) {
            LOSE_SCORE
        } else {
            DRAW_SCORE
        }
    }

    fn match_score(&self, opponent: Self) -> i64 {
        self.game_score(opponent) + self.shape_score()
    }
}

