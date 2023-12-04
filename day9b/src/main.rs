use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn get_adjacent_position(&self, other: &Position) -> Position {
        if (self.x - other.x).abs() < 2 && (self.y - other.y).abs() < 2 {
            return Position { ..*other };
        }
        if self.x == other.x {
            if self.y > other.y {
                return Position::new(self.x, self.y - 1);
            } else {
                return Position::new(self.x, self.y + 1);
            }
        }
        if self.y == other.y {
            if self.x > other.x {
                return Position::new(self.x - 1, self.y);
            } else {
                return Position::new(self.x + 1, self.y);
            }
        }
        match (self.x > other.x, self.y > other.y) {
            (true, true) => Position::new(other.x + 1, other.y + 1),
            (true, false) => Position::new(other.x + 1, other.y - 1),
            (false, true) => Position::new(other.x - 1, other.y + 1),
            (false, false) => Position::new(other.x - 1, other.y - 1),
        }
    }

    fn get_next_position(&self, direction: &str) -> Position {
        match direction {
            "U" => Position::new(self.x + 1, self.y),
            "D" => Position::new(self.x - 1, self.y),
            "L" => Position::new(self.x, self.y - 1),
            "R" => Position::new(self.x, self.y + 1),
            _ => panic!("Invalid input"),
        }
    }
}
fn main() {
    let input = include_str!("../input.txt");
    let mut positions: Vec<Position> = vec![Position::new(0, 0); 10];
    let mut set: HashSet<Position> = HashSet::new();
    set.insert(Position::new(0, 0));
    for (direction, moves) in input.lines().map(|line| line.split_once(' ').unwrap()) {
        let moves: u32 = moves.parse().unwrap();
        for _ in 0..moves {
            positions[0] = positions[0].get_next_position(direction);
            for i in 1..positions.len() {
                let next_position = positions[i - 1].get_adjacent_position(&positions[i]);
                if positions[i] == next_position {
                    break;
                }
                positions[i] = next_position;
            }
            set.insert(positions[9]);
        }
    }
    println!("{}", set.len());
}
