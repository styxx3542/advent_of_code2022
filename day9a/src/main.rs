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

    fn get_adjacent_position(&self, other: &Position, prev: &Position) -> Position {
        if (self.x - other.x).abs() < 2 && (self.y - other.y).abs() < 2 {
            return Position { ..*other };
        }
        Position { ..*prev }
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
    let mut set: HashSet<Position> = HashSet::new();
    let mut head_position = Position::new(0, 0);
    let mut tail_position = Position::new(0, 0);
    set.insert(tail_position);
    for (direction, moves) in input.lines().map(|line| line.split_once(' ').unwrap()) {
        let moves: u32 = moves.parse().unwrap();
        for _ in 0..moves {
            let prev = Position { ..head_position };
            head_position = head_position.get_next_position(direction);
            tail_position = head_position.get_adjacent_position(&tail_position, &prev);
            set.insert(tail_position);
        }
    }
    println!("{}", set.len());
}
