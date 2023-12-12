use nom::{
    bytes::complete::tag, character::complete as cc, multi::separated_list1, sequence::tuple,
    IResult,
};
use std::collections::HashSet;
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn make_set(v: Vec<Vec<Coord>>) -> HashSet<Coord> {
        let mut s = HashSet::new();
        for rocks in v.iter() {
            let mut prev = rocks[0];
            for next in rocks.iter() {
                for c in prev.draw_line(next) {
                    s.insert(c);
                }
                prev = next.clone();
            }
        }
        s
    }

    pub fn get_next_coord(&self, set: &HashSet<Coord>) -> Self {
        let dir = vec![(0, 1), (-1, 1), (1, 1)];
        for (dx, dy) in dir.iter() {
            let next = Coord {
                x: self.x + *dx,
                y: self.y + *dy,
            };
            if !set.contains(&next) {
                return next;
            }
        }
        self.clone()
    }

    fn draw_line(&self, other: &Self) -> Vec<Coord> {
        let mut v = vec![];
        if self.x == other.x {
            for y in self.y.min(other.y)..=self.y.max(other.y) {
                v.push(Coord { x: self.x, y })
            }
        } else {
            for x in self.x.min(other.x)..=self.x.max(other.x) {
                v.push(Coord { x, y: self.y });
            }
        }
        v
    }
    pub fn is_out_of_bounds(&self) -> bool {
        self.x.abs() > 30 || self.y.abs() > 300
    }
}
//513,151 -> 513,155 -> 510,155 -> 510,161 -> 519,161 -> 519,155 -> 515,155 -> 515,151
fn parse_rocks(s: &str) -> IResult<&str, Vec<Coord>> {
    let (s, v) = separated_list1(tag(" -> "), tuple((cc::i32, tag(","), cc::i32)))(s)?;
    Ok((
        s,
        v.into_iter()
            .map(|(x, _, y)| Coord { x: x - 500, y })
            .collect(),
    ))
}

pub fn parse_input(s: &str) -> IResult<&str, Vec<Vec<Coord>>> {
    separated_list1(cc::multispace1, parse_rocks)(s)
}
