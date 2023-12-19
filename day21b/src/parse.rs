use std::collections::HashMap;

use nom::{
    branch::alt, bytes::complete::tag, character::complete as cc, combinator::value,
    multi::separated_list1, sequence::tuple, IResult,
};
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Node {
    Leaf(Leaf),
    Inner(Inner),
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Leaf {
    pub val: i64,
    pub id: String,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Inner {
    pub id: String,
    pub left: String,
    pub right: String,
    pub operation: Operation,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Operation {
    Plus,
    Minus,
    Div,
    Mult,
}

impl Operation {
    fn eval(&self, left: i64, right: i64) -> i64 {
        match *self {
            Operation::Plus => left + right,
            Operation::Minus => left - right,
            Operation::Div => left / right,
            Operation::Mult => left * right,
        }
    }

    pub fn from_result(&self, result: i64, left: i64, right: i64) -> i64 {
        if left == i64::MAX {
            match *self {
                Operation::Plus => result - right,
                Operation::Minus => result + right,
                Operation::Div => result * right,
                Operation::Mult => result / right,
            }
        } else {
            match *self {
                Operation::Plus => result - left,
                Operation::Minus => left - result,
                Operation::Div => left / result,
                Operation::Mult => result / left,
            }
        }
    }
}

impl Node {
    pub fn eval(&self, map: &HashMap<String, Node>) -> i64 {
        match self {
            Node::Leaf(leaf) => leaf.val,
            Node::Inner(inner) => {
                let left = map.get(&inner.left).unwrap();
                let right = map.get(&inner.right).unwrap();
                inner.operation.eval(left.eval(&map), right.eval(&map))
            }
        }
    }

    pub fn find(&self, map: &HashMap<String, Node>, target: String) -> Vec<String> {
        match self {
            Node::Leaf(leaf) => {
                if leaf.id == target {
                    return vec![target];
                }
                vec![]
            }
            Node::Inner(inner) => {
                let mut left = map.get(&inner.left).unwrap().find(&map, target.clone());
                let mut right = map.get(&inner.right).unwrap().find(&map, target.clone());
                if left.len() > 0 {
                    left.push(inner.id.clone());
                    return left;
                }
                if right.len() > 0 {
                    right.push(inner.id.clone());
                    return right;
                }
                vec![]
            }
        }
    }
}
//bppv: 8
//jdjd: lzrs * trhh

fn parse_operation(s: &str) -> IResult<&str, Operation> {
    alt((
        value(Operation::Plus, tag("+")),
        value(Operation::Minus, tag("-")),
        value(Operation::Mult, tag("*")),
        value(Operation::Div, tag("/")),
    ))(s)
}

fn parse_line(s: &str) -> IResult<&str, Node> {
    alt((parse_leaf, parse_inner))(s)
}

fn parse_leaf(s: &str) -> IResult<&str, Node> {
    let (s, (id, _, val)) = tuple((cc::alpha1, tag(": "), cc::i64))(s)?;
    let leaf = Leaf { id: id.into(), val };
    Ok((s, Node::Leaf(leaf)))
}

fn parse_inner(s: &str) -> IResult<&str, Node> {
    let (s, (id, _, left, _, operation, _, right)) = tuple((
        cc::alpha1,
        tag(": "),
        cc::alpha1,
        cc::multispace1,
        parse_operation,
        cc::multispace1,
        cc::alpha1,
    ))(s)?;
    let inner = Inner {
        id: id.into(),
        left: left.into(),
        right: right.into(),
        operation,
    };
    Ok((s, Node::Inner(inner)))
}

pub fn parse_input(s: &str) -> IResult<&str, HashMap<String, Node>> {
    let (s, v) = separated_list1(cc::multispace1, parse_line)(s)?;
    let mut map = HashMap::new();
    for node in v.into_iter() {
        match node {
            Node::Inner(inner) => map.insert(inner.id.clone(), Node::Inner(inner)),
            Node::Leaf(leaf) => map.insert(leaf.id.clone(), Node::Leaf(leaf)),
        };
    }
    Ok((s, map))
}
