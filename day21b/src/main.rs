mod parse;
use nom::{combinator::all_consuming, Finish};
use parse::{parse_input, Node};
use std::collections::HashMap;
fn evaluate(mut path: Vec<String>, result: i64, map: &HashMap<String, Node>) -> i64 {
    if path.len() == 1 {
        return result;
    }
    let id = path.pop().unwrap();
    let node = map.get(&id).unwrap();
    if let Node::Inner(inner) = node {
        let next = path.last().unwrap();
        if inner.left == *next {
            let other = map.get(&inner.right).unwrap().eval(&map);
            let result = inner.operation.from_result(result, i64::MAX, other);
            return evaluate(path, result, map);
        }
        let other = map.get(&inner.left).unwrap().eval(&map);
        let result = inner.operation.from_result(result, other, i64::MAX);
        return evaluate(path, result, map);
    }
    0
}
fn main() {
    let map = all_consuming(parse_input)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    let root = map.get("root".into()).unwrap();
    let mut path = root.find(&map, String::from("humn"));
    path.pop();
    if let Node::Inner(inner) = root {
        let next = path.last().unwrap();
        let result;
        if inner.left == *next {
            result = map.get(&inner.right).unwrap().eval(&map);
        } else {
            result = map.get(&inner.left).unwrap().eval(&map);
        }
        println!("{}", evaluate(path, result, &map));
    }
}
