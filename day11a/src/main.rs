use nom::{combinator::all_consuming, Finish};
mod parse;
use parse::{parse_all_monkeys, Monkey};

fn turn(mut monkey: Monkey, monkeys: &mut Vec<Monkey>) -> Monkey {
    for item in monkey.items.into_iter() {
        monkey.items_inspected += 1;
        let val = monkey.operation.eval(item) / 3;
        if val % monkey.divisor == 0 {
            monkeys[monkey.receiver_if_true].items.push(val);
        } else {
            monkeys[monkey.receiver_if_false].items.push(val);
        }
    }
    monkey.items = Vec::new();
    monkey
}
fn main() {
    let mut monkeys = all_consuming(parse_all_monkeys)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i] = turn(monkeys[i].clone(), &mut monkeys);
        }
    }
    monkeys.sort_by_key(|monkey| monkey.items_inspected);
    let val =
        monkeys[monkeys.len() - 1].items_inspected * monkeys[monkeys.len() - 2].items_inspected;
    println!("{val}");
}
