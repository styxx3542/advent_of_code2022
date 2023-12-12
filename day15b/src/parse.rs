use nom::{
    bytes::complete::tag, character::complete as cc, multi::separated_list1, sequence::tuple,
    IResult,
};
//Sensor at x=9450, y=2172986: closest beacon is at x=-657934, y=1258930
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn col_range(&self, target_row: i32, other: &Self) -> Option<(i32, i32)> {
        let distance = self.manhattan_distance(other);
        let distance_range = distance - (self.y - target_row).abs();
        if distance_range < 0 {
            return None;
        }
        let left = (self.x - distance_range).max(0);
        let right = (self.x + distance_range).min(4_000_000);
        Some((left, right))
    }
}

fn parse_sensors(s: &str) -> IResult<&str, (Position, Position)> {
    let (s, (_, x1, _, y1, _, x2, _, y2)) = tuple((
        tag("Sensor at x="),
        cc::i32,
        tag(", y="),
        cc::i32,
        tag(": closest beacon is at x="),
        cc::i32,
        tag(", y="),
        cc::i32,
    ))(s)?;
    Ok((s, (Position::new(x1, y1), Position::new(x2, y2))))
}

pub fn parse_input(s: &str) -> IResult<&str, Vec<(Position, Position)>> {
    separated_list1(cc::multispace1, parse_sensors)(s)
}
