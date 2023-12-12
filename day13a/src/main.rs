mod parse;
use nom::{combinator::all_consuming, Finish};
use parse::{parse_input, Element};
fn main() {
    let pairs = all_consuming(parse_input)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    let i: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, _)| i + 1)
        .sum();
    println!("{i}");
}
