mod parse;
use nom::{combinator::all_consuming, Finish};
use parse::{parse_input, Element};
fn main() {
    let mut packets =
        all_consuming(parse_input)(concat!(include_str!("../input.txt"), "\n[[2]]\n[[6]]"))
            .finish()
            .unwrap()
            .1;
    packets.sort();
    let l = packets
        .binary_search(&vec![Element::List(vec![Element::Integer(2)])])
        .unwrap();
    let r = packets
        .binary_search(&vec![Element::List(vec![Element::Integer(6)])])
        .unwrap();
    println!("{}", (l + 1) * (r + 1));
}
