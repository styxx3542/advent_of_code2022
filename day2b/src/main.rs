#[derive(PartialEq, Clone)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_str(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            "_" => panic!("unexpected input"),
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            Outcome::Draw => 3,
            Outcome::Win => 6,
            Outcome::Lose => 0,
        }
    }
}

impl Moves {
    fn from_str(s: &str) -> Moves {
        match s {
            "A" => Moves::Rock,
            "B" => Moves::Paper,
            "C" => Moves::Scissors,
            _ => panic!("Unexpected input {s}"),
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            Moves::Rock => 1,
            Moves::Paper => 2,
            Moves::Scissors => 3,
        }
    }

    fn get_opponent_move(&self, outcome: &Outcome) -> Move {
        match outcome {
            Outcome::Draw => self.clone(),
            Outcome::Lose => match self {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Outcome::Win => match self {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
        }
    }

    fn calculate_score(&self, opponent: &Moves) -> u32 {
        if self == opponent {
            return 3 + opponent.get_score();
        } else {
            match (self, opponent) {
                (Moves::Rock, Moves::Scissors)
                | (Moves::Paper, Moves::Rock)
                | (Moves::Scissors, Moves::Paper) => 6 + self.get_score(),
                _ => 0 + self.get_score(),
            }
        }
    }
}
fn main() {
    let v = include_str!("../input.txt")
        .split("\n")
        .map(|s| {
            s.split(' ')
                .map(|s| Moves::from_str(s))
                .collect::<Vec<Moves>>()
        })
        .map(|v| v[1].calculate_score(&v[0]))
        .sum();
    println!("{}", val);
    }