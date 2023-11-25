#[derive(PartialEq, Clone)]
enum Move {
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
            _ => panic!("unexpected input"),
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

impl Move {
    fn from_str(s: &str) -> Move {
        match s {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => panic!("Unexpected input {s}"),
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn get_move(&self, outcome: &Outcome) -> Move {
        match outcome {
            Outcome::Draw => self.clone(),
            Outcome::Lose => match self {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
            Outcome::Win => match self {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
        }
    }
}
fn main() {
    let v: u32 = include_str!("../input.txt")
        .split("\n")
        .map(|s| {
            let (a, b) = s.split_once(' ').unwrap();
            let (opponent_move, outcome) = (Move::from_str(a), Outcome::from_str(b));
            let my_move = opponent_move.get_move(&outcome);
            my_move.get_score() + outcome.get_score()
        })
        .sum();

    println!("{v}");
}
