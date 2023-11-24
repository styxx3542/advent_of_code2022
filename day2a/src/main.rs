#[derive(PartialEq)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    fn from_str(s: &str) -> Moves {
        match s {
            "A" | "X" => Moves::Rock,
            "B" | "Y" => Moves::Paper,
            "C" | "Z" => Moves::Scissors,
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
    let val: u32 = include_str!("../input.txt")
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
