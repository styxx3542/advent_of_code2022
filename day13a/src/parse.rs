use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as cc,
    character::complete::{one_of, space0, space1},
    combinator::{map, value},
    error::ParseError,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};
use std::cmp::Ordering;
#[derive(PartialEq, Eq)]
pub enum Element {
    Integer(u32),
    List(Vec<Element>),
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Element::Integer(l), Element::Integer(r)) => l.cmp(&r),
            (Element::List(l), Element::List(r)) => l.cmp(&r),
            (Element::Integer(l), Element::List(r)) => vec![Element::Integer(*l)].cmp(&r),
            (Element::List(l), Element::Integer(r)) => l.cmp(&vec![Element::Integer(*r)]),
        }
    }
}

/*
[[],[[],[[5,4]],10],[1]]
[[[],3,1,[[4],3,0,[2,2]]],[[[0,3,4]],[0,2,[4,10,1,5]]],[],[6,[],9]]
*/

fn parse_list(s: &str) -> IResult<&str, Vec<Element>> {
    let (s, (_, v, _)) = tuple((tag("["), separated_list0(tag(","), parse_element), tag("]")))(s)?;
    Ok((s, v))
}

fn parse_element(s: &str) -> IResult<&str, Element> {
    alt((
        map(cc::u32, Element::Integer),
        map(parse_list, Element::List),
    ))(s)
}

fn parse_packet(s: &str) -> IResult<&str, (Vec<Element>, Vec<Element>)> {
    let (s, l) = parse_list(s)?;
    let (s, r) = preceded(tag("\n"), parse_list)(s)?;
    Ok((s, (l, r)))
}
pub fn parse_input(s: &str) -> IResult<&str, Vec<(Vec<Element>, Vec<Element>)>> {
    separated_list1(cc::multispace1, parse_packet)(s)
}
