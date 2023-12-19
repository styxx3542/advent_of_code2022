mod parse;
use nom::{combinator::all_consuming, Finish};
use parse::{parse_input, Inner, Leaf, Node, Operation};
fn main() {
    let mut map = all_consuming(parse_input)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    let root = map.get("root".into()).unwrap();
    println!("{}", root.eval(&map));
}
