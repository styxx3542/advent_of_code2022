use core::panic;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete as cc,
    character::complete::{one_of, space0, space1},
    combinator::{map, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
#[derive(Debug, Clone)]
pub struct Monkey {
    pub items_inspected: u64,
    pub items: Vec<u64>,
    pub operation: Operation,
    pub divisor: u64,
    pub receiver_if_true: usize,
    pub receiver_if_false: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(Term, Term),
    Mul(Term, Term),
}

impl Operation {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.eval(old) + r.eval(old),
            Operation::Mul(l, r) => l.eval(old) * r.eval(old),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Term {
    Old,
    Constant(u64),
}

impl Term {
    pub fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Constant(c) => c,
        }
    }
}

/*
Monkey 0:
  Starting items: 66, 79
  Operation: new = old * 11
  This is a line from vim keybinding in vscode. Test: divisible by 7
    If true: throw to monkey 6
    If false: throw to monkey 7
 */

pub fn parse_term(i: &str) -> IResult<&str, Term> {
    alt((value(Term::Old, tag("old")), map(cc::u64, Term::Constant)))(i)
}

pub fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let (i, (l, _, op, _, r)) = preceded(
        tag("new = "),
        tuple((parse_term, space1, one_of("*+"), space1, parse_term)),
    )(i)?;
    let op = match op {
        '*' => Operation::Mul(l, r),
        '+' => Operation::Add(l, r),
        _ => panic!("Error"),
    };
    Ok((i, op))
}
fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    let (i, _) = tuple((tag("Monkey "), cc::u64, tag(":\n")))(i)?;
    let (i, (_, _, items, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), cc::u64),
        tag("\n"),
    ))(i)?;
    let (i, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, tag("\n")))(i)?;
    let (i, (_, _, divisor, _)) =
        tuple((space1, tag("Test: divisible by "), cc::u64, tag("\n")))(i)?;
    let (i, (_, _, receiver_if_true, _)) = tuple((
        space1,
        tag("If true: throw to monkey "),
        map(cc::u64, |x| x as usize),
        tag("\n"),
    ))(i)?;
    let (i, (_, _, receiver_if_false, _)) = tuple((
        space1,
        tag("If false: throw to monkey "),
        map(cc::u64, |x| x as usize),
        tag("\n"),
    ))(i)?;
    Ok((
        i,
        Monkey {
            items_inspected: 0,
            items,
            operation,
            divisor,
            receiver_if_true,
            receiver_if_false,
        },
    ))
}

pub fn parse_all_monkeys(i: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(cc::multispace1, parse_monkey)(i)
}
