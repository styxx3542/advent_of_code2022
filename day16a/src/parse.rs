use nom::{
    branch::alt, bytes::complete::tag, character::complete as cc, multi::separated_list1,
    sequence::tuple, IResult,
};
use std::collections::HashMap;
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Node {
    pub id: String,
    pub flow_rate: u32,
    pub neighbors: Vec<String>,
    pub index: usize,
}

//Valve ED has flow rate=0; tunnels lead to valves PS, AW
fn parse_valves(s: &str) -> IResult<&str, Node> {
    let (s, (_, id, _, flow_rate, _, v)) = tuple((
        tag("Valve "),
        cc::alpha1,
        tag(" has flow rate="),
        cc::u32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), cc::alpha1),
    ))(s)?;
    let node = Node {
        id: id.into(),
        flow_rate,
        neighbors: v.into_iter().map(|s| s.to_string()).collect(),
        index: 0,
    };
    Ok((s, node))
}

pub fn parse_input(s: &str) -> IResult<&str, HashMap<usize, Node>> {
    let mut map = HashMap::new();
    let (s, v) = separated_list1(cc::multispace1, parse_valves)(s)?;
    for (i, mut node) in v.into_iter().enumerate() {
        node.index = i;
        map.insert(node.index, node);
    }
    Ok((s, map))
}
